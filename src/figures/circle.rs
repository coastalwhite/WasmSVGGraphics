use std::hash::{Hash, Hasher};

pub struct CircleProps {
    radius: u32
}

impl CircleProps {
    pub fn new(radius: u32) -> CircleProps {
        CircleProps {
            radius
        }
    }

    pub fn to_element(&self) -> web_sys::Element {
        let circle = crate::create_element_ns(crate::SVG_NS, "circle")
            .expect("Failed to create circle object!");
        circle.set_attribute("r", &self.radius.to_string()[..])
            .expect("Cannot attach r to circle");
        circle.set_attribute("cx", "0")
            .expect("Cannot attach cx to circle");
        circle.set_attribute("cy", "0")
            .expect("Cannot attach cy to circle");

        circle
    }
}

impl Hash for CircleProps {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.radius.hash(state);
    }
}