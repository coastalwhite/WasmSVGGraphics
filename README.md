# WasmSVGGraphics
A Rust library for rendering SVG Graphics with WASM

This crate provides a fast and effective way to interact with SVG's using WebAssembly.
It is able to:
* Declare shapes and styles for these shapes for later use
* Render these shapes to the DOM using defintions
* Automatically detect if two shapes are the same, so only one defintion will get added to the DOM
* Declare named items for later adjustments

# Note
This crate is still under development, but most API calls for 1.0.0 are completed.
If any bugs are found please submit a issue or a pull request at:
[GitHub](https://github.com/coastalwhite/WasmSVGGraphics)

# Further notice
The *-dev* versions are purely for testing and should not serve as