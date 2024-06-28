bindings:
	poetry run componentize-py --wit-path wit --world logcraft:lgc/plugins@0.1.0 bindings myplugin

build:
	poetry run componentize-py --wit-path wit --world logcraft:lgc/plugins@0.1.0 componentize -p myplugin main -o my-plugin.wasm

clear:
	rm -f my-plugin.wasm
