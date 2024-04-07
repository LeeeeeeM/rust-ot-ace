# test
cargo test --test single_tests

# build 
wasm-pack build ot-wasm (when use it, remove `init` fn in the main.tsx)

# build for web
wasm-pack build ot-wasm --target=web

# dev 
yarn link (execute in pkg)

# doc for bindgen
https://rustwasm.github.io/wasm-bindgen/contributing/design/exporting-rust.html