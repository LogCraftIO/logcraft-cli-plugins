bindings:
	rm -rf plugins
	poetry run componentize-py --wit-path ../wit --world logcraft:lgc/plugins@0.1.0 bindings .

build:
	poetry run componentize-py --wit-path ../wit --world logcraft:lgc/plugins@0.1.0 componentize main -o my-plugin.wasm