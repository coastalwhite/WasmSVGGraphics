use crate::figures::sub_path::SubPath;
use geom_2d::point::Point;
use std::hash::{Hash, Hasher};

/// Structure to represent a SVG Path tag
pub struct PathProps {
    /// Point from which to start the path
    start_point: Point,

    /// All the subpaths from the 'start_point' forward
    sub_paths: Vec<SubPath>,

    /// Should the path be closed
    closed: bool,
}

impl PathProps {
    pub fn new(start_point: Point, sub_paths: Vec<SubPath>, closed: bool) -> PathProps {
        PathProps {
            start_point,
            sub_paths,
            closed,
        }
    }

    fn to_d_string(&self) -> String {
        let mut d_string = format!("M {} {}", self.start_point.x(), self.start_point.y());

        for sub_path in self.sub_paths.iter() {
            d_string.push_str(&format!(" {}", sub_path.to_string())[..]);
        }

        if self.closed {
            d_string.push_str(" Z");
        }

        d_string
    }

    pub fn to_element(&self) -> web_sys::Element {
        let path = crate::create_element_ns(crate::SVG_NS, "path")
            .expect("Failed to create path element!");
        path.set_attribute("d", &self.to_d_string()[..])
            .expect("Cannot attach d to path");

        path
    }
}

impl Hash for PathProps {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start_point.hash(state);
        self.sub_paths.iter().for_each(|x| x.hash(state));
        self.closed.hash(state);
    }
}
