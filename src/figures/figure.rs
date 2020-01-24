use crate::color::TransparentableColor;
use crate::color;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use geom_2d::point::Point;

#[derive(Hash)]
pub struct Shape {
    style: FigureStyle,
    figure: Box<dyn Figureable>
}

impl Shape {
    pub fn to_styled_element(&self) -> web_sys::Element {
        let element = self.figure.to_element();

        self.style.apply_style(&element);

        element
    }
}

pub enum AttributeField {
    StrokeWidth(u32),
    StrokeColor(TransparentableColor),
    FillColor(TransparentableColor)
}

impl AttributeField {
    pub fn add_to_element(&self, element: &web_sys::Element) {
        match self {
            AttributeField::StrokeWidth(width) => Self::set_stroke_width(element, width),
            AttributeField::StrokeColor(color) => Self::set_stroke_color(element, color),
            AttributeField::FillColor(color) => Self::set_fill_color(element, color),
        }
    }

    fn set_stroke_width(element: &web_sys::Element, width: &u32) {
        element.set_attribute("stroke", &width.to_string()[..]);
    }

    fn set_stroke_color(element: &web_sys::Element, color: &TransparentableColor) {
        element.set_attribute("stroke", &color.to_string()[..]);
    }

    fn set_fill_color(element: &web_sys::Element, color: &TransparentableColor) {
        element.set_attribute("stroke", &color.to_string()[..]);
    }
}

#[derive(Hash)]
pub struct FigureStyle {
    attributes: Vec<AttributeField>
}

const DEFAULT_STROKE_WIDTH: u32 = 1;
const DEFAULT_STROKE_COLOR: TransparentableColor = TransparentableColor::Color(color::default::BLACK);
const DEFAULT_FILL_COLOR: TransparentableColor = TransparentableColor::Transparent;

impl FigureStyle {
    pub fn new_from_default() -> FigureStyle {
        FigureStyle {
            attributes: vec![
                AttributeField::StrokeWidth(DEFAULT_STROKE_WIDTH),
                AttributeField::StrokeColor(DEFAULT_STROKE_COLOR),
                AttributeField::FillColor(DEFAULT_FILL_COLOR)
            ]
        }
    }

    pub fn new() -> FigureStyle {
        FigureStyle {
            attributes: vec![]
        }
    }

    pub fn add_style(&mut self, new_attribute: AttributeField) {
        self.attributes.push(new_attribute);
    }

    pub fn apply_style(&self, element: &web_sys::Element) {
        self.attributes.iter().for_each(|x| x.add_to_element(element));
    }
}

impl Hash for dyn Figureable {
    fn hash(&self, state: &mut DefaultHasher) {

    }
}

pub enum SubShape {
    Path(PathProps),
    Circle(CircleProps)
}

impl SubShape {
    fn to_element(&self) -> web_sys::Element;
    fn is_fillable(&self) -> bool;
}