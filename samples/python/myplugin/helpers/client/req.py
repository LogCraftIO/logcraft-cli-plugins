"""Module with helpers for wasi http"""

import asyncio
import traceback
import poll_loop
from poll_loop import PollLoop, Sink, Stream

from plugins.types import Ok, Err
from plugins.imports.types import (
    IncomingResponse, Method, Method_Get, Method_Head, Method_Post, Method_Put, Method_Delete, Method_Connect, Method_Options,
    Method_Trace, Method_Patch, Method_Other, IncomingRequest, IncomingBody, ResponseOutparam, OutgoingResponse,
    Fields, Scheme, Scheme_Http, Scheme_Https, Scheme_Other, OutgoingRequest, OutgoingBody
)
from plugins.imports.streams import StreamError_Closed

from dataclasses import dataclass
from collections.abc import MutableMapping
from typing import Optional
from urllib import parse

@dataclass
class Request:
    """An HTTP request"""
    method: str
    uri: str
    headers: MutableMapping[str, str]
    body: Optional[bytes]

@dataclass
class Response:
    """An HTTP response"""
    status: int
    headers: MutableMapping[str, str]
    body: Optional[bytes]

def send(request: Request) -> Response:
    """Send an HTTP request and return a response or raise an error"""
    loop = PollLoop()
    asyncio.set_event_loop(loop)
    return loop.run_until_complete(send_async(request))
    

async def send_async(request: Request) -> Response:
    match request.method:
        case "GET":
            method: Method = Method_Get()
        case "HEAD":
            method = Method_Head()
        case "POST":
            method = Method_Post()
        case "PUT":
            method = Method_Put()
        case "DELETE":
            method = Method_Delete()
        case "CONNECT":
            method = Method_Connect()
        case "OPTIONS":
            method = Method_Options()
        case "TRACE":
            method = Method_Trace()
        case "PATCH":
            method = Method_Patch()
        case _:
            method = Method_Other(request.method)
    
    url_parsed = parse.urlparse(request.uri)

    match url_parsed.scheme:
        case "http":
            scheme: Scheme = Scheme_Http()
        case "https":
            scheme = Scheme_Https()
        case _:
            scheme = Scheme_Other(url_parsed.scheme)

    if request.headers.get('content-length') is None:
        content_length = len(request.body) if request.body is not None else 0
        request.headers['content-length'] = str(content_length)

    headers = list(map(
        lambda pair: (pair[0], bytes(pair[1], "utf-8")),
        request.headers.items()
    ))

    outgoing_request = OutgoingRequest(Fields.from_list(headers))
    outgoing_request.set_method(method)
    outgoing_request.set_scheme(scheme)
    outgoing_request.set_authority(url_parsed.netloc)
    path_and_query = url_parsed.path
    if url_parsed.query:
        path_and_query += '?' + url_parsed.query
    outgoing_request.set_path_with_query(path_and_query)

    outgoing_body = request.body if request.body is not None else bytearray()
    sink = Sink(outgoing_request.body())
    incoming_response: IncomingResponse = (await asyncio.gather(
        poll_loop.send(outgoing_request),
        send_and_close(sink, outgoing_body)
    ))[0]

    response_body = Stream(incoming_response.consume())
    body = bytearray()
    while True:
        chunk = await response_body.next()
        if chunk is None:
            simple_response = Response(
                incoming_response.status(),
                dict(map(
                    lambda pair: (pair[0], str(pair[1], "utf-8")),
                    incoming_response.headers().entries()
                )),
                bytes(body)
            )
            incoming_response.__exit__()
            return simple_response
        else:
            body += chunk

async def send_and_close(sink: Sink, data: bytes):
    await sink.send(data)
    sink.close()
