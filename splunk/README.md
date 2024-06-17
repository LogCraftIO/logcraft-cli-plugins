# Splunk plugin

The Splunk plugin allows integrating LogCraft CLI with Cisco Splunk.


## Getting the bits

### Releases

This is the recommended approach for most users, directly [download the latest build](https://github.com/LogCraftIO/logcraft-cli-plugins/releases) of the plugin.

### Building from the sources

If you prefer, you can build the plugin from the sources:

```bash
git clone https://github.com/LogCraftIO/logcraft-cli-plugins
cd logcraft-cli-plugins
cd splunk
cargo component build --release
```

The plugin will be released under: `../target/wasm32-wasi/release/splunk.wasm`. Add it to `lgc` using the `plugins install` command:

```bash
~$ cd your-work-dir
~$ lgc plugins install /path/to/target/wasm32-wasi/release/splunk.wasm
```

Note that compiling the plugin requires `cargo-component` and `wasm32-wasi`:

```bash
cargo install cargo-component --locked
rustup target add wasm32-wasi
```

## Installing the plugin
Once instantiated as a service, default values will be set.

## Configuration

The plugin has 2 parameters:

- `endpoint`: defines the URL of the Splunk server to interact with
- `authorization`: defines the authorization mechanism to use (Bearer or Basic)

### Authorization

#### User tokens
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
    authorization: Bearer eyJraWQiOiJzcGx1bmsuc2VjcmV0IiwiYW.....z4IaBtAHPFg
```

**Make sure to include the keyword `Bearer` before the token as illustrated above.**

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
    authorization: Basic YndheW5lOmJhdG1hbg==
```

Similarely to tokens, make sure to add the keyword `Basic` before the base64 encoding.