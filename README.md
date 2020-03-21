# WasmSVGGraphics

A Rust library for rendering SVG Graphics with WASM

This crate provides a fast and effective way to interact with SVG's using WebAssembly.
It is able to:

-   Declare shapes and styles to use for these shapes
-   Render these shapes to the DOM using the SVG _\<def\>_ tag
-   Automatically detect if two shapes are the same, so only a single SVG _\<def\>_ will get added to the DOM
-   Declare named items/containers for later adjustments, such as hiding, reshowing and repositioning

# Changelog

## 1.0.2

-   Added support for _svg_definitions 0.3.0_, which introduces a feature to parse files and pure strings into svg elements.
-   Added `new_from_svg`, which lets you create a svg from SVGElem.

## 1.0.1

-   Added support for _svg_definitions 0.2.0_, which introduces more svg elements and attributes.

# Note

Version 1.0.1 is tested to be stable, and can be used in development.

This crate is still under development, but most API calls for 1.0.0 are complete.
If any bugs are found please submit a issue or a pull request at:
[GitHub](https://github.com/coastalwhite/WasmSVGGraphics)

# Further notice

The _-dev_ versions are purely for testing and should not serve as production or development versions.

# Testing

When working on this crate, some testing was done of the documentation using the [wasm_bindgen_test](https://crates.io/crates/wasm-bindgen-test) crate. These tests can be found in the github under _/tests/web.rs_ and can be executed with [wasm_pack](https://github.com/rustwasm/wasm-pack) using the command `wasm-pack test --headless --firefox --chrome --safari`
