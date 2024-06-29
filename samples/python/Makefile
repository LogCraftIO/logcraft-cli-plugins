PLUGIN_FILE=my-plugin.wasm
PLUGIN_FOLDER=myplugin

schemas:
	poetry run python3 myplugin/helpers/schemas.py

bindings:
	poetry run componentize-py --wit-path wit --world logcraft:lgc/plugins@0.1.0 bindings ${PLUGIN_FOLDER}

build:
	make clear
	make schemas
	poetry run componentize-py --wit-path wit --world logcraft:lgc/plugins@0.1.0 componentize -p ${PLUGIN_FOLDER} main -o ${PLUGIN_FILE}

clear:
	rm -f ${PLUGIN_FILE}
	rm -rf ${PLUGIN_FOLDER}/plugins
	rm -rf ${PLUGIN_FOLDER}/schemas