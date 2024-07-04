# Splunk plugin

The Splunk plugin allows integrating LogCraft CLI with Cisco Splunk.

## Installation

For installation instructions, please refer to the [root README](../README.md).

## Configuration

The plugin has the following parameters:

- `endpoint`: defines the URL of the Splunk server to interact with
- `authorization_scheme`: defines the authorization mechanism to use: Bearer (recommanded) or Basic.
- `authorization`: set the token to use, either a JWT Token (Bearer) or a Base64 encoded string `base64(user:password)` (Basic).
- `timeout`: an optional timeout for the communications with Splunk, default to 60 seconds.

### Authorization

#### JWT/User tokens (recommended)
Authentication tokens are the recommended mechanism to authenticate to Splunk.
Log in to Splunk with admin privileges, then go to **Settings > Tokens** and create a new token.

Fill in the form as follows:
- User: &lt;your user&gt;
- Audience: logcraft-cli
- Expiration: as it fits your needs

Then click **Create** and copy the token.


```bash
~$ cat lgc.yaml
...
services:
- name: splunk-dev
  plugin: splunk
  settings:
    endpoint: https://192.168.64.22:8089
    authorization_scheme: Bearer
    authorization: eyJraWQiOiJzcGx1bmsuc2VjcmV0IiwiYW.....z4IaBtAHPFg
```

#### Basic

**Avoid using Basic authentification**, prefer using user tokens, but if you still need/want to do it, here is the procedure.

Convert your credentials `username:password` in based64:

```bash
~$ echo -n "bwayne:batman" | base64
YndheW5lOmJhdG1hbg==
~$
```

Set the authorization accordingly:

```bash
~$ cat lgc.yaml
...
services:
- name: splunk-dev
  plugin: splunk
  settings:
    endpoint: https://192.168.64.22:8089
    authorization_scheme: Basic
    authorization: YndheW5lOmJhdG1hbg==
```
