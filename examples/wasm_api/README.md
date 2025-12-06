# WASM bindings (wasm-bindgen)

Build bindings (web + node):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --example wasm_api --features wasm-example

wasm-bindgen --target web --out-dir examples/wasm/pkg \
  target/wasm32-unknown-unknown/debug/examples/wasm_api.wasm

wasm-bindgen --target nodejs --out-dir examples/wasm/pkg-node \
  target/wasm32-unknown-unknown/debug/examples/wasm_api.wasm
```

Browser demo:

```bash
cd examples/wasm
python -m http.server 8989
# open http://localhost:8989
```

Node CLI using wasm bindings (after generating pkg-node):

```bash
node examples/wasm/cli.js price --spot 105 --strike 100 --mat 0.25 --rate 0.03 --div 0.01 --vol 0.22
node examples/wasm/cli.js iv --price 4.25 --spot 102 --strike 100 --mat 0.25 --rate 0.02 --div 0.0
```

More details in `examples/wasm/README.md`.
