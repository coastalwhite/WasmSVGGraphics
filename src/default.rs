//! Contains some easy and nice ways to create definitions and shapes to render

use svg_definitions::prelude::*;

/// Creates a default circle with a certain radius
pub fn circle(radius: i32) -> SVGElem {
    SVGElem::new(Tag::Circle)
        .set(Attr::Radius, radius.into())
        .set(Attr::StrokeColor, RGB::new(0, 0, 0).into())
        .set(Attr::StrokeWidth, 1.into())
        .set(Attr::FillColor, RGBT::Transparent.into())
        .set(Attr::CenterX, 0.into())
        .set(Attr::CenterY, 0.into())
}

/// Creates a default rectangle with a certain width and height
pub fn rect(width: i32, height: i32) -> SVGElem {
    SVGElem::new(Tag::Rectangle)
        .set(Attr::Width, width.into())
        .set(Attr::Height, height.into())
        .set(Attr::StrokeColor, RGB::new(0, 0, 0).into())
        .set(Attr::StrokeWidth, 1.into())
        .set(Attr::FillColor, RGBT::Transparent.into())
}

/// Creates a default curve with control points 1 and 2 and an end point
pub fn curve(
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
    cx1: i32,
    cy1: i32,
    cx2: i32,
    cy2: i32,
) -> SVGElem {
    SVGElem::new(Tag::SVGPath).set(
        Attr::PathDefinition,
        PathString::new()
            .move_to(as_point_2d((sx, sy)))
            .curve_to(
                as_point_2d((ex, ey)),
                as_point_2d((cx1, cy1)),
                as_point_2d((cx2, cy2)),
            )
            .into(),
    )
}

/// Creates a polygon from a vector of points
pub fn polygon(points: Vec<(i32, i32)>) -> SVGElem {
    let mut path_string = PathString::new().move_to(as_point_2d(points[0]));

    points[1..]
        .iter()
        .for_each(|point| path_string = path_string.clone().line_to(as_point_2d(*point)));

    SVGElem::new(Tag::SVGPath)
        .set(Attr::PathDefinition, path_string.into())
        .set(Attr::StrokeWidth, 1.into())
        .set(Attr::StrokeColor, RGB::new(0, 0, 0).into())
}

/// Sets the location of SVG elem (for circles use [set_circle_loc](#set_circle_loc))
pub fn set_loc(elem: SVGElem, x: i32, y: i32) -> SVGElem {
    elem.set(Attr::PositionX, x.into())
        .set(Attr::PositionY, y.into())
}

/// Sets the location of SVG Circle (for non circles use [set_loc](#set_loc))
pub fn set_circle_loc(elem: SVGElem, x: i32, y: i32) -> SVGElem {
    elem.set(Attr::CenterX, x.into())
        .set(Attr::CenterY, y.into())
}

fn as_point_2d(point: (i32, i32)) -> Point2D {
    (point.0 as f64, point.1 as f64)
}
