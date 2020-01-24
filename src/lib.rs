/// Module containing the renderer and its logic
pub mod renderer;

/// Module containing all preset figures and traits to define new ones
pub mod figures;

/// Module containing definition for Color abstraction used and some preset colors
pub mod color;


const SVG_NS: &str = "http://www.w3.org/2000/svg";

pub fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("No window exists");
    window.document().expect("The windows should contains a document")
}

pub fn create_element(local_name: &str) -> web_sys::Element {
    get_document()
        .create_element(local_name)
        .expect("Could not create new element")
}

pub fn create_element_ns(namespace: &str, name: &str) -> web_sys::Element {
    get_document()
        .create_element_ns(Some(namespace), name)
        .expect("Could not create new namespaced element")
}