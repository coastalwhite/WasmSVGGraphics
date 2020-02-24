//! This crate provides a fast and effective way to interact with SVG's using WebAssembly.
//! It is able to:
//! * Declare shapes and styles for these shapes for later use
//! * Render these shapes to the DOM using defintions
//! * Automatically detect if two shapes are the same, so only one defintion will get added to the DOM
//! * Declare named items for later adjustments
//!
//! # Note
//! This crate is still under development, but most API calls for 1.0.0 are completed.
//! If any bugs are found please submit a issue or a pull request at:
//! [GitHub](https://github.com/coastalwhite/WasmSVGGraphics)
//!
//! # Roadmap
//! * Create more options for shapes, and create a easier way to declare these
//! * Create more styling options and make these dynamic with subshapes
//!
//! # Examples
//! ## Basics (How to render a circle)
//! ```
//! use wasm_svg_graphics::figures;
//! use geom_2d::point::Point;
//! use wasm_svg_graphics::renderer::Renderer;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = Renderer::new("svg_parent_id")
//!     .expect("Failed to create renderer!");
//!
//! // Generate circle
//! let circle = figures::preset::circle(10);
//!
//! // Render circle (since it's the first time of rendering this shape,
//! // the renderer will add the shape's definition)
//! renderer.render(&circle, Point::new(20, 20));
//! ```
//!
//! As one can see, it's not that difficult render a circle to the svg
//!
//! ## Basics (How to render a custom shape)
//! ```
//! use wasm_svg_graphics::figures::Figure;
//! use wasm_svg_graphics::figures::shape::*;
//! use wasm_svg_graphics::figures::circle::CircleProps;
//! use wasm_svg_graphics::figures::path::PathProps;
//! use wasm_svg_graphics::figures::sub_path::SubPath;
//! use geom_2d::point::Point;
//! use wasm_svg_graphics::renderer::Renderer;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = Renderer::new("svg_parent_id")
//!     .expect("Failed to create renderer!");
//!
//! let style = ShapeStyle::new_from_default();
//!
//! // Generate smiley
//! let style = ShapeStyle::new_from_default();
//!
//! let smiley = Figure::new(vec![
//!    // Head
//!    (
//!        Shape::new(style.clone(), SubShape::Circle(CircleProps::new(20))),
//!        Point::new(0, 0),
//!    ),
//!    // Left eye
//!    (
//!        Shape::new(style.clone(), SubShape::Circle(CircleProps::new(3))),
//!        Point::new(-7, -7),
//!    ),
//!    // Right eye
//!    (
//!        Shape::new(style.clone(), SubShape::Circle(CircleProps::new(3))),
//!        Point::new(7, -7),
//!    ),
//!    // Mouth
//!    (
//!        Shape::new(
//!            style.clone(),
//!            SubShape::Path(PathProps::new(
//!                Point::new(-7, 0), // Beginning point
//!                vec![SubPath::new_bezier_curve(
//!                    // Create a new curve
//!                    Point::new(-4, 5), // Control point 1
//!                    Point::new(4, 5),  // Control point 2
//!                    Point::new(7, 0),  // Ending point
//!                )],
//!                false, // Is path closed?
//!             )),
//!        ),
//!        Point::new(0, 5),
//!    ),
//! ]);
//!
//! renderer.render(&smiley, Point::new(25, 25));
//! ```
//!
//! Declaring custom figures is maybe somewhat of a cumbersome tasks but most definitely worth it!
//!
//! ## Basics (How to render with custom style)
//! Let's use the smiley example from before, but now color it yellow
//! ```
//! use wasm_svg_graphics::figures::Figure;
//! use wasm_svg_graphics::figures::shape::*;
//! use wasm_svg_graphics::figures::circle::CircleProps;
//! use wasm_svg_graphics::figures::path::PathProps;
//! use wasm_svg_graphics::figures::sub_path::SubPath;
//! use geom_2d::point::Point;
//! use wasm_svg_graphics::renderer::Renderer;
//! use wasm_svg_graphics::color::Color;
//!
//! // Declare renderer (must be mutable) into a parent container
//! let mut renderer = Renderer::new("svg_parent_id")
//!     .expect("Failed to create renderer!");
//!
//! // Create head style
//! let mut yellow_stroke = ShapeStyle::new_from_default();
//!
//! // Assign a yellow stroke color
//! yellow_stroke.add_style(
//!     AttributeField::StrokeColor,
//!     Color::new(255,255,0).to_string()
//! );
//!
//! // Create eye style
//! let mut black_fill = ShapeStyle::new_from_default();
//!
//! // Assign a yellow stroke color
//! black_fill.add_style(
//!     AttributeField::FillColor,
//!     Color::new(0,0,0).to_string()
//! );
//!
//! // Create head style
//! let mut red_stroke = ShapeStyle::new_from_default();
//!
//! // Assign a yellow stroke color
//! red_stroke.add_style(
//!     AttributeField::StrokeColor,
//!     Color::new(255,0,0).to_string()
//! );
//!
//! // Generate smiley
//! let smiley = Figure::new(vec![
//!    // Head
//!    (
//!        Shape::new(yellow_stroke.clone(), SubShape::Circle(CircleProps::new(20))),
//!        Point::new(0, 0),
//!    ),
//!    // Left eye
//!    (
//!        Shape::new(black_fill.clone(), SubShape::Circle(CircleProps::new(3))),
//!        Point::new(-7, -7),
//!    ),
//!    // Right eye
//!    (
//!        Shape::new(black_fill.clone(), SubShape::Circle(CircleProps::new(3))),
//!        Point::new(7, -7),
//!    ),
//!    // Mouth
//!    (
//!        Shape::new(
//!            red_stroke.clone(),
//!            SubShape::Path(PathProps::new(
//!                Point::new(-7, 0), // Beginning point
//!                vec![SubPath::new_bezier_curve(
//!                    // Create a new curve
//!                    Point::new(-4, 5), // Control point 1
//!                    Point::new(4, 5),  // Control point 2
//!                    Point::new(7, 0),  // Ending point
//!                )],
//!                false, // Is path closed?
//!             )),
//!        ),
//!        Point::new(0, 5),
//!    ),
//! ]);
//!
//! renderer.render(&smiley, Point::new(25, 25));
//! ```

use crate::errors::DomError::*;
use crate::errors::RendererError;
use crate::errors::RendererError::*;

use svg_definitions::prelude::*;

/// Container for the actual renderer object, this includes all logic for adding items to the DOM and for detecting duplication
pub mod renderer;

/// Container with all the errors, mostly used internally
pub mod errors;

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

    elem
}
