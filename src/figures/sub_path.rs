use geom_2d::point::Point;
use std::hash::{Hash, Hasher};

pub enum SubPath {
    /// (Control point 1, Control point 2, End point)
    BezierCurve(Point, Point, Point),
    /// (End Point)
    Line(Point),
}

impl SubPath {
    /// Creates new line with ending point at 'end_point'
    pub fn new_line(end_point: Point) -> SubPath {
        SubPath::Line(end_point)
    }

    /// Creates new bezier curve with control points 1 and 2 and ending point at 'end_point'
    pub fn new_bezier_curve(
        control_point1: Point,
        control_point2: Point,
        end_point: Point,
    ) -> SubPath {
        SubPath::BezierCurve(control_point1, control_point2, end_point)
    }

    fn line_string(ep: &Point) -> String {
        format!("L {} {}", ep.x(), ep.y())
    }

    fn bezier_curve_string(c1: &Point, c2: &Point, ep: &Point) -> String {
        format!(
            "C {} {} {} {} {} {}",
            c1.x(),
            c1.y(),
            c2.x(),
            c2.y(),
            ep.x(),
            ep.y()
        )
    }

    /// Returns its contribution to the d attribute
    pub fn to_string(&self) -> String {
        match self {
            BezierCurve(c1, c2, ep) => SubPath::bezier_curve_string(c1, c2, ep),
            Line(ep) => SubPath::line_string(ep),
        }
    }
}

use SubPath::*;

impl Hash for SubPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Line(p1) => p1.hash(state),
            BezierCurve(p1, p2, p3) => {
                p1.hash(state);
                p2.hash(state);
                p3.hash(state);
            },
        }
    }
}