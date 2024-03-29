# test
cargo test --test single_test

# build 
wasm-pack build rust-wasm

# build for web
wasm-pack build rust-wasm --target=web

# doc for bindgen
https://rustwasm.github.io/wasm-bindgen/contributing/design/exporting-rust.html