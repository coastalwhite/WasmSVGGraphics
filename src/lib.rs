//! This crate provides a fast and effective way to interact with SVG's using WebAssembly.
//! It is able to:
//! * Declare shapes and styles for these shapes for later use
//! * Render these shapes to the DOM using defintions
//! * Automatically detect if two shapes are the same, so only one defintion will get added to the DOM
//! * Declare named items for later adjustments
//!
//! # SVG Definitions
//! To define custom shapes and for all the documentation have a look at the [svg_definitions](https://crates.io/crates/svg_definitions) crate.
//!
//! # Note
//! This crate is still under development, but most API calls for 1.0.0 are completed.
//! If any bugs are found please submit a issue or a pull request at:
//! [GitHub](https://github.com/coastalwhite/WasmSVGGraphics)
//!
//! # Examples
//! ## Basics (How to render a circle)
//! ```rust,no_run
//! use wasm_svg_graphics::prelude::*;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");
//!
//! // Generate circle
//! let circle = SVGDefault::circle(10);
//!
//! // Render circle (since it's the first time of rendering this shape,
//! // the renderer will add the shape's definition)
//! renderer.render(circle, (20.0, 20.0));
//! ```
//!
//! As one can see, it's not that difficult render a circle to the svg
//!
//! ## Basics (How to render a custom shape)
//! ```rust,no_run
//! use wasm_svg_graphics::prelude::*;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = SVGRenderer::new("svg_parent_id")
//!     .expect("Failed to create renderer!");
//!
//! let smiley = SVGElem::new(Tag::G)
//!     .append(SVGDefault::circle(20))
//!     .append(SVGDefault::set_circle_loc(SVGDefault::circle(3), -7, -7))
//!     .append(SVGDefault::set_circle_loc(SVGDefault::circle(3), 7, -7))
//!     .append(SVGDefault::curve(-7, 5, 7, 5, -4, 10, 4, 10));
//!
//! renderer.render(smiley, (25.0, 25.0));
//! ```
//!
//! Declaring custom figures is maybe somewhat of a cumbersome tasks but most definitely worth it!
//!
//! ## Basics (How to render with custom style)
//! Let's use the smiley example from before, but now color it yellow
//! ```rust,no_run
//! use wasm_svg_graphics::prelude::*;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = SVGRenderer::new("svg_parent_id")
//!     .expect("Failed to create renderer!");
//!
//! let colored_smiley = SVGElem::new(Tag::G)
//!     .append(SVGDefault::circle(20).set(Attr::Stroke, "#ffff00"))
//!     .append(
//!         SVGDefault::set_circle_loc(SVGDefault::circle(3), -7, -7)
//!             .set(Attr::Fill, "#000000"),
//!     )
//!     .append(
//!         SVGDefault::set_circle_loc(SVGDefault::circle(3), 7, -7)
//!             .set(Attr::Fill, "#000000"),
//!     )
//!     .append(
//!         SVGDefault::curve(-7, 5, 7, 5, -4, 10, 4, 10)
//!             .set(Attr::Stroke, "#ff0000")
//!             .set(Attr::Fill, "transparent"),
//!     );
//!
//! renderer.render(colored_smiley, (25.0, 25.0));
//! ```

use crate::errors::DomError::*;
use crate::errors::RendererError;
use crate::errors::RendererError::*;

use svg_definitions::prelude::*;

pub mod default;
mod errors;
pub mod prelude;
pub mod renderer;

const NAME_ID_PREFIX: &str = "named";
const SHAPE_ID_PREFIX: &str = "figure";
const SVG_NS: &str = "http://www.w3.org/2000/svg";

fn get_document() -> Result<web_sys::Document, RendererError> {
    let window = web_sys::window().ok_or(Dom(NoWindow))?;

    window.document().ok_or(Dom(NoDocument))
}

fn create_element_ns(namespace: &str, name: &str) -> Result<web_sys::Element, RendererError> {
    get_document()?
        .create_element_ns(Some(namespace), name)
        .map_err(|_| Dom(UncreatableNSElement))
}

fn to_html(svg_elem: &SVGElem) -> web_sys::Element {
    let elem = create_element_ns(SVG_NS, &svg_elem.get_tag_name().to_string()[..])
        .expect("Failed to create element");

    svg_elem.get_attributes().iter().for_each(|(attr, value)| {
        elem.set_attribute(&attr.to_string()[..], &value.to_string()[..])
            .expect("Failed to set attribute");
    });

    svg_elem.get_children().iter().for_each(|child| {
        elem.append_child(&to_html(child))
            .expect("Failed to append child");
    });

    if let Some(inner_html) = svg_elem.get_inner() {
        elem.set_inner_html(inner_html);
    }

    elem
}
