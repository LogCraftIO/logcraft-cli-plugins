# Python Plugin Sample

This is an example plugin using python.

---
**Documentation**: <a href="https://docs.logcraft.io" target="_blank">https://docs.logcraft.io</a>
---

## Poetry

This example uses python poetry to manage dependencies but feel free to use your prefered package manager.

## Build

Building a plugin is a 2 steps process:

1. Build the bindings for your IDE. This is optional but advised for development.

```bash
poetry run componentize-py --wit-path wit --world plugins bindings myplugin

```

2. Build the plugin. This step automatically build the bindings regardless if you did it in the previous step or not. The resulting wasm file is the LogCraft CLI plugin.

```bash
poetry run componentize-py --wit-path wit --world plugin componentize -p myplugin main -o my-plugin.wasm
```


## Important

As of june 2024, `componentize-py` uses cpython runtime without `zlib`. This is an issue that has consequences: we cannot use python `requests` library and probably others.

This is a known and identified problem that will be fixed in the future by the `componentize-py` team.

To workaround this issue, we created a http library based on `sink`, this is available in this sample python application (`myplugin.client`).


## WIT

The wit files provides from:

1. `wit/world.wit` and `wit/plugin.wit` are LogCraft specific configuration files. These files define input/outputs of plugins.
2. `wit/deps/` come from [wasi-http](https://github.com/WebAssembly/wasi-http/tree/main/wit/deps)