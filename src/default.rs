//! Contains some easy and nice ways to create definitions and shapes to render

use svg_definitions::prelude::*;

/// Creates a default circle with a certain radius
pub fn circle(radius: i32) -> SVGElem {
    SVGElem::new(Tag::Circle)
        .set(Attr::Radius, radius)
        .set(Attr::Stroke, "#000000")
        .set(Attr::StrokeWidth, 1)
        .set(Attr::Fill, "transparent")
        .set(Attr::Cx, 0)
        .set(Attr::Cy, 0)
}

/// Creates a default rectangle with a certain width and height
pub fn rect(width: i32, height: i32) -> SVGElem {
    SVGElem::new(Tag::Rect)
        .set(Attr::Width, width)
        .set(Attr::Height, height)
        .set(Attr::Stroke, "#000000")
        .set(Attr::StrokeWidth, 1)
        .set(Attr::Fill, "transparent")
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
    SVGElem::new(Tag::Path).set(
        Attr::D,
        PathData::new().move_to(as_point_2d((sx, sy))).curve_to(
            as_point_2d((ex, ey)),
            as_point_2d((cx1, cy1)),
            as_point_2d((cx2, cy2)),
        ),
    )
}

/// Creates a polygon from a vector of points
pub fn polygon(points: Vec<(i32, i32)>) -> SVGElem {
    let mut path_string = PathData::new().move_to(as_point_2d(points[0]));

    points[1..]
        .iter()
        .for_each(|point| path_string = path_string.clone().line_to(as_point_2d(*point)));

    SVGElem::new(Tag::Path)
        .set(Attr::D, path_string)
        .set(Attr::StrokeWidth, 1)
        .set(Attr::Stroke, "#000000")
}

/// Sets the location of SVG elem (for circles use [set_circle_loc](#set_circle_loc))
pub fn set_loc(elem: SVGElem, x: i32, y: i32) -> SVGElem {
    elem.set(Attr::X, x).set(Attr::Y, y)
}

/// Sets the location of SVG Circle (for non circles use [set_loc](#set_loc))
pub fn set_circle_loc(elem: SVGElem, x: i32, y: i32) -> SVGElem {
    elem.set(Attr::Cx, x).set(Attr::Cy, y)
}

fn as_point_2d(point: (i32, i32)) -> Point2D {
    (point.0 as f32, point.1 as f32)
}
