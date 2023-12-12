# `#![no_std]` hello world for wasm32-wasi target

Look ma, no `std`!

## Building

```console
cargo build --target wasm32-wasi
```

## Running

```console
cd target/wasm32-wasi/debug
wasmtime --dir=. wasi_hello_world.wasm
```

## Backstory

It seems that the process of building for `wasm32-wasi` has changed over time. I had to combine approaches from

1. ["Stubbing out WASI manually in Rust"][stubbing_out_wasi_manually_in_rust]
2. ["`#![no_std]` with WASI is more complicated than I thought it would be"][no_std_with_wasi] article on Medium;
3. ["Adding experimental WebAssembly support to Decaton - Part 2"][wasm_support_to_decathon] article from LINE Engineering Blog;
4. ["WASM Micro Runtime with Rust"][wasm_micro_runtime] article by Anoop Alias.

[stubbing_out_wasi_manually_in_rust]: https://www.jakubkonka.com/2020/04/28/rust-wasi-from-scratch.html
[no_std_with_wasi]: https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7
[wasm_support_to_decathon]: https://engineering.linecorp.com/en/blog/adding-experimental-webassembly-support-to-decaton-part-2
[wasm_micro_runtime]: https://anoopelias.github.io/posts/wasm-micro-runtime-with-rust/
