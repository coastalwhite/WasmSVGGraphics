use crate::color::TransparentableColor;
use crate::color;
use std::hash::{Hash, Hasher};
use crate::figures::shape::AttributeField::StrokeWidth;

pub struct Shape {
    style: ShapeStyle,
    subshape: SubShape
}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.style.hash(state);
        self.subshape.hash(state);
    }
}

impl Shape {
    pub fn new(style: ShapeStyle, subshape: SubShape) -> Shape {
        Shape {
            style,
            subshape
        }
    }

    pub fn to_styled_element(&self) -> web_sys::Element {
        let element = self.subshape.to_element();

        self.style.apply_style(&element);

        element
    }
}

#[derive(Hash, PartialEq)]
pub enum AttributeField {
    StrokeWidth,
    StrokeColor,
    FillColor
}

impl AttributeField {
    pub fn set_attribute(&self, element: &web_sys::Element, value: &str) {
        element
            .set_attribute(self.to_attribute_string(), value)
            .expect("Unable to set attribute of Shape");
    }

    fn to_attribute_string(&self) -> &str {
        match self {
            AttributeField::StrokeWidth => "stroke-width",
            AttributeField::StrokeColor => "stroke",
            AttributeField::FillColor => "fill"
        }
    }
}

pub struct ShapeStyle {
    attributes: Vec<(AttributeField, String)>
}

impl Hash for ShapeStyle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attributes.iter().for_each(|x| x.hash(state));
    }
}

const DEFAULT_STROKE_WIDTH: u32 = 1;
const DEFAULT_STROKE_COLOR: TransparentableColor = TransparentableColor::RGB(color::default::BLACK);
const DEFAULT_FILL_COLOR: TransparentableColor = TransparentableColor::Transparent;

use AttributeField::*;
use crate::figures::shape::SubShape::Path;

impl ShapeStyle {
    pub fn new_from_default() -> ShapeStyle {
        ShapeStyle {
            attributes: vec![
                (StrokeWidth, DEFAULT_STROKE_WIDTH.to_string()),
                (StrokeColor, DEFAULT_STROKE_COLOR.to_string()),
                (FillColor, DEFAULT_FILL_COLOR.to_string())
            ]
        }
    }

    pub fn new() -> ShapeStyle {
        ShapeStyle {
            attributes: vec![]
        }
    }

    pub fn add_style(&mut self, attribute: AttributeField, value: String) {
        let duplicate = self.attributes
            .iter_mut()
            .find(
                |x|
                    x.0 == attribute
            ); // Find item with same AttributeField

        match duplicate {
            None => self.attributes.push((attribute, value)),
            Some(dupl) => *dupl = (attribute, value)
        }
    }

    pub fn apply_style(&self, element: &web_sys::Element) {
        self.attributes
            .iter()
            .for_each(
                |x|
                    x.0.set_attribute(element, &x.1[..])
            );
    }
}

use SubShape::*;
use crate::figures::path::PathProps;
use crate::figures::circle::CircleProps;

pub enum SubShape {
    Path(PathProps),
    Circle(CircleProps)
}

impl Hash for SubShape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Path(props) => props.hash(state),
            Circle(props) => props.hash(state),
        }
    }
}

impl SubShape {
    fn to_element(&self) -> web_sys::Element {
        match self {
            Path(props) => props.to_element(),
            Circle(props) => props.to_element(),
        }
    }
}