use crate::errors::RendererError;
use crate::errors::DomError::*;
use crate::errors::RendererError::*;

/// Module containing the renderer and its logic
pub mod renderer;

/// Module containing all preset figures and traits to define new ones
pub mod figures;

/// Module containing definition for Color abstraction used and some preset colors
pub mod color;

/// Module containing all the errors
pub mod errors;

const NAME_ID_PREFIX: &str = "named";
const SHAPE_ID_PREFIX: &str = "shape";
const SVG_NS: &str = "http://www.w3.org/2000/svg";

fn get_document() -> Result<web_sys::Document, RendererError> {
    let window = web_sys::window()
        .ok_or(Dom(NoWindow))?;

    window.document()
        .ok_or(Dom(NoDocument))
}

/*fn create_element(local_name: &str) -> Result<web_sys::Element, RendererError> {
    get_document()?
        .create_element(local_name)
        .map_err(|_| Dom(UncreatableElement))
}*/

fn create_element_ns(namespace: &str, name: &str) -> Result<web_sys::Element, RendererError> {
    get_document()?
        .create_element_ns
        (
            Some(namespace),
            name
        )
        .map_err(|_| Dom(UncreatableNSElement))
}