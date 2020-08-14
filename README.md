# rgltf

This unfinished library has Rust bindings to the [cgltf](https://github.com/jkuhlmann/cgltf/) library.

The goal is to also have a safe wrapper around [src/ffi.rs](./src/ffi.rs) in [src/lib.rs](./src/lib.rs).

There's one example that parses a glTF file and prints the output.

`cargo run --example parse_file`
