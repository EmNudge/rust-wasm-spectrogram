all: wasm_project

wasm_project:
	wasm-pack build

serve:
	npx serve .