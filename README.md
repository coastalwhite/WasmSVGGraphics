# WasmSVGGraphics

A Rust library for rendering SVG Graphics with WASM

This crate provides a fast and effective way to interact with SVG's using WebAssembly.
It is able to:

-   Declare shapes and styles to use for these shapes
-   Render these shapes to the DOM using the SVG _\<def\>_ tag
-   Automatically detect if two shapes are the same, so only a single SVG _\<def\>_ will get added to the DOM
-   Declare named items/containers for later adjustments, such as hiding, reshowing and repositioning

# Note

Version 0.3.0 is tested to be stable, and can be used in development.

This crate is still under development, but most API calls for 1.0.0 are complete.
If any bugs are found please submit a issue or a pull request at:
[GitHub](https://github.com/coastalwhite/WasmSVGGraphics)

# Further notice

The _-dev_ versions are purely for testing and should not serve as production or development versions.
