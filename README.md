# LogCraft CLI plugins mono-repository

This repository contains plugins for LogCraft CLI. Pull/merge requests are welcome and encouraged!

---

**Documentation**: <a href="https://docs.logcraft.io" target="_blank">https://docs.logcraft.io</a>

**LogCraft CLI**: <a href="https://github.com/LogCraftIO/logcraft-cli" target="_blank">https://github.com/LogCraftIO/logcraft-cli</a>

---

## LogCraft CLI

LogCraft CLI is an open-source tool developed by [LogCraft](https://www.logcraft.io) that simplifies the creation of Detection-as-Code pipelines while leveraging native Version Control System (VCS) capabilities such as GitLab.

With LogCraft CLI, you can easily deploy your security detections into your SIEM, EDR, XDR, and other modern security solutions.

## Plugins

- [Splunk](./splunk)
- [Microsoft Azure Sentinel](./sentinel)

## Getting the bits

### Releases

[Download the latest build](https://github.com/LogCraftIO/logcraft-cli-plugins/releases) of the desired plugins directly from the releases page. This is the recommended approach for most users.

### Building from the sources

If you prefer, you can build the plugins from the sources:

First, clone the repository:

```bash
git clone https://github.com/LogCraftIO/logcraft-cli-plugins
cd logcraft-cli-plugins
```

Then, enter the directory of the plugin of your choice

```bash
cd <PLUGIN_DIR>
cargo component build --release
```

For example
```bash
cd splunk
cargo component build --release
```

The plugin will be released under: `../target/wasm32-wasi/release/` as a `.wasm` file. 

Add it to `lgc` using the `plugins install` command:

```bash
~$ cd your-work-dir
~$ lgc plugins install /path/to/target/wasm32-wasi/release/<PLUGIN>.wasm
```
For example:

```bash
~$ lgc plugins install /path/to/target/wasm32-wasi/release/splunk.wasm
```

Note that compiling the plugin requires `cargo-component` and `wasm32-wasi`:

```bash
cargo install cargo-component --locked
rustup target add wasm32-wasi
```
