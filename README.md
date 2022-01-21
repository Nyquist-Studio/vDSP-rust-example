# Calling vDSP in Rust

Minimal examples of calling [vDSP](https://developer.apple.com/documentation/accelerate/vdsp?language=objc) functions in Rust.

# High Level Steps

At a high level, calling vDSP functions in Rust (on macOS) require these steps:

1. Link the final executable (e.g. an executable target or a dynamic library) to `Accelerate.framework` (see [build.rs](./build.rs)) for how to do this in Rust
2. Declare the vDSP functions of interest in Rust as `extern "C"` functions, you will also need to map all types in the original C interface into corresponding Rust types (see [main.rs](./src/main.rs)) for examples
3. Simply call the functions you just declared in Rust code, note that since those functions are C functions, they are `unsafe`

# rust-bindgen Possible?

I also attempted to use [rust-bindgen](https://github.com/rust-lang/rust-bindgen) to generate the `extern "C"` function declarations (i.e. Rust bindings of the original C functions) automatically, but didn't find an easy way to get the header search path correct (since the `vDSP.h` header file includes some headers from other system frameworks such as `CoreFoundation`).
