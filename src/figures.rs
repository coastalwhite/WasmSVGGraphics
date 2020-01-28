/// Module containing the definition for Shape, ShapeStyle, AttributeField
pub mod shape;

/// Module containing PathProps (The properties used when creating a Shape::Path)
pub mod path;
/// Module containing CircleProps (The properties used when creating a Shape::Circle)
pub mod circle;

/// Module containing the definition of SubPath which is used for defining smaller parts of a whole Shape::Path
pub mod sub_path;



use shape::Shape;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A combination of shapes into one object used as a svg-def
#[derive(Hash)]
pub struct Figure {
    shapes: Vec<Shape>
}

impl Figure {
    pub fn new(shapes: Vec<Shape>) -> Figure {
        Figure {
            shapes
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    /// Returns the id used for this Figure's definition
    pub fn get_id(&self) -> String {
        let hash = self.get_hash();
        format!("{}-{}", super::SHAPE_ID_PREFIX, format!("{:x}", hash))
    }

    pub fn to_def(&self) -> web_sys::Element {
        let id = self.get_id();

        let g_element = crate::create_element_ns(crate::SVG_NS, "g");
        g_element.set_id(&id[..]);

        for shape in self.shapes.iter() {
            g_element
                .append_child(&shape.to_styled_element())
                .expect("Cant append shape to figure");
        }

        g_element
    }
}

/// A set of presets for Figure, e.g. lines, circles, ...
pub mod preset {
    use super::Figure;
    use crate::figures::shape::{ Shape, ShapeStyle, SubShape };
    use crate::figures::circle::CircleProps;
    use crate::figures::path::PathProps;
    use crate::figures::sub_path::SubPath;
    use geom_2d::point::Point;

    pub fn circle(radius: u32) -> Figure {
        Figure::new(
            vec![
                Shape::new(
                    ShapeStyle::new_from_default(),
                    SubShape::Circle(
                        CircleProps::new(radius)
                    )
                )
            ]
        )
    }

    pub fn line(start_point: Point, end_point: Point) -> Figure {
        Figure::new(
            vec![
                Shape::new(
                    ShapeStyle::new_from_default(),
                    SubShape::Path(
                        PathProps::new(
                            start_point,
                            vec![SubPath::Line(end_point)],
                            false
                        )
                    )
                )
            ]
        )
    }
}