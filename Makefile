# Build helpers for WASM bindings from the `wasm_api` example.

WASM_EXAMPLE=wasm_api
WASM_FEATURES=wasm-example
WASM_TARGET=wasm32-unknown-unknown

.PHONY: wasm-bindings wasm-clean

# Build web (plain JS) and bundler/React bindings into target/wasm/
wasm-bindings:
	wasm-pack build --target web --out-dir target/wasm/pkg-web -- \
	  --example $(WASM_EXAMPLE) --features $(WASM_FEATURES)
	wasm-pack build --target bundler --out-dir target/wasm/pkg-react -- \
	  --example $(WASM_EXAMPLE) --features $(WASM_FEATURES)
	mkdir -p target/wasm/pkg-react/web
	cp target/wasm/pkg-web/quant_opts* target/wasm/pkg-react/web/

wasm-clean:
	rm -rf target/wasm/pkg-web target/wasm/pkg-react
