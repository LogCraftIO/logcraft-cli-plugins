# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0

from typing import Optional

# bindings generated by `componentize-py`
from myplugin.plugins import Plugins
from myplugin.plugins.types import Err, Ok, Some, Result
from myplugin.plugins.exports import Plugin
from myplugin.plugins.exports.plugin import Metadata

# As of June 2024, the `requests` library is not supported due to missing dependencies
# in the cpython runtime used by componentize-py (ssl support, zlib).
#
# A fix is planned for the future, so in the mean time, we use our own http library
# derivated from sink
# https://github.com/bytecodealliance/componentize-py/issues/96

from myplugin.client.req import Request, Response, send

class Plugin(Plugin):
    # func() -> metadata;
    def load(self) -> Metadata:
        """
        The `load()` function is called when the plugin is installed using `lgc plugins install`.

        It should return a `Metadata` object containing the plugin's name, version, author, and description.
        Make sure the name respect kebab-case (lowercase and separated by dashes). The provided information
        will be displayed in the lgc.yaml file.
        """
        return Metadata("my-plugin", "0.1.0", "LogCraft", "This is a famous plugin")

    # func() -> string;
    def settings(self) -> str:
        return  "OK"

    # func() -> string;
    def schema(self) -> str:
        return  "OK"

    # func(config: string, name: string, params: string) -> result<option<string>, string>;
    def create(self, config: str, name: str, params: str) -> Result[Optional[str], str]:
        return Ok(Some("create()"))

    # func(config: string, name: string, params: string) -> result<option<string>, string>;
    def read(self, config: str, name: str, params: str) -> Optional[str]:
       return Ok(Some("read()"))

    # func(config: string, name: string, params: string) -> result<option<string>, string>;
    def update(self, config: str, name: str, params: str) -> Optional[str]:
        return Ok(Some("update()"))

    # func(config: string, name: string, params: string) -> result<option<string>, string>;
    def delete(self, config: str, name: str, params: str) -> Optional[str]:
        return Ok(Some("delete()"))

    # ping: func(config: string) -> result<bool, string>;
    def ping(self, config: str) -> int: 
        try:
            resp = send(Request("GET", "https://google.fr", {}, None))
        except Exception as e:
            raise Err(str(e))
        
        if resp.status_code >= 400:
            raise Err(str(resp.status_code))

        return Ok(resp.status_code)
