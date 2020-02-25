//! Contains some useful definitions for SVG Graphics, also re-exports svg_definitions

pub use crate::default as SVGDefault;
pub use crate::renderer::Renderer as SVGRenderer;
pub use svg_definitions::prelude::*;

SVGElem::new(Tag::Group).append(
    SVGDefault::set_loc(SVGDefault::circle(20), 0, 0)
    .set(Attr::StrokeColor, RGB::new(255, 255, 0).into())
).append(
    SVGDefault::set_loc(SVGDefault::circle(3), -7, -7)
    .set(Attr::FillColor, RGB::new(0, 0, 0).into())
).append(
    SVGDefault::set_loc(SVGDefault::circle(3), 7, -7)
    .set(Attr::FillColor, RGB::new(0, 0, 0).into())
).append(
    SVGDefault::set_loc(SVGDefault::curve(7, 5, -4, 10, 4, 40), -7, 5)
    .set(Attr::StrokeColor, RGB::new(255, 0, 0).into())
)