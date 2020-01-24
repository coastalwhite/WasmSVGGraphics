use std::collections::BTreeSet;
use crate::figures::Figure;
use geom_2d::point::Point;

/// Container object used to interact with the SVG Object
/// Keeps track of definitions and dom root id
pub struct Renderer {
    /// The id of the SVG element within the dom
    dom_root_id: String,

    /// All the already defined SVG definitions
    shape_defs: BTreeSet<u64>
}

impl Renderer {
    /// Create new renderer object
    ///
    /// # Arguments
    /// **dom_root_id**: *the id of the svg*
    pub fn new(dom_root_id: &str) -> Renderer {
        let document = crate::get_document();

        let root = document
            .get_element_by_id(dom_root_id).expect("Unable to find root from id");

        let svg_element = crate::create_element_ns("http://www.w3.org/2000/svg", "svg");
        let defs_element = crate::create_element_ns("http://www.w3.org/2000/svg", "defs");
        svg_element.append_child(&defs_element).expect("Unable to append defs to svg");
        root.append_child(&svg_element).expect("Unable to create svg to root");

        Renderer {
            dom_root_id: String::from(dom_root_id),
            shape_defs: BTreeSet::new()
        }
    }

    fn get_root(&self) -> web_sys::Element {
        let document = crate::get_document();
        document
            .get_element_by_id(&self.dom_root_id[..])
            .expect("The Renderer root could not be found")
    }

    fn get_svg_root(&self) -> web_sys::Element {
        let svg = self.get_root().first_element_child();

        match svg {
            None => panic!("No svg element was found: Renderer root has no children."),
            Some(root) => {
                if root.tag_name() != "svg" {
                    panic!("No svg element was found: first child is not svg");
                }

                root
            }
        }
    }

    fn get_defs_root(&self) -> web_sys::Element {
        let defs = self.get_svg_root().first_element_child();

        match defs {
            None => panic!("No defs element was found: SVG root has no children."),
            Some(root) => {
                if root.tag_name() != "defs" {
                    panic!("No defs element was found: first child is not defs");
                }

                root
            }
        }
    }

    fn contains_shape(&self, shape: &Figure) -> bool {
        let hash = shape.get_hash();

        self.shape_defs.contains(&hash)
    }

    fn add_def(&mut self, shape: &Figure) {
        self.get_defs_root()
            .append_child(&shape.to_def())
            .expect("Unable to add definition of Figure");

        let hash = shape.get_hash();
        self.shape_defs.insert(hash);
    }

    fn add_use(&self, def_id: &str, location: Point) {
        let root = self.get_root();
        let use_element = crate::create_element_ns(crate::SVG_NS, "use");

        use_element
            .set_attribute("href", &format!("#{}", def_id)[..])
            .expect("Couldn't set href of use element");

        use_element
            .set_attribute("x", &format!("{}", location.x())[..])
            .expect("Error setting x of use tag");
        use_element
            .set_attribute("y", &format!("{}", location.y())[..])
            .expect("Error setting y of use tag");

        root
            .append_child(&use_element)
            .expect("Unable to add use of Figure");
    }

    /// Render figure at location, this will automatically add a definition
    ///
    /// # Arguments
    /// **figure**: *A ref to a wasm_svg_graphics::figures::Figure object*<br/>
    /// **location**: *the location where to add the use tag*
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_svg_graphics::figures::Figure;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = wasm_svg_graphics::renderer::new("svg_id");
    ///
    /// // Generate circle
    /// let circle = Figure::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render(circle, Point::new(20, 20);
    /// ```
    pub fn render(&mut self, figure: &Figure, location: Point) {
        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure);
        }

        // Add use of definition
        self.add_use(&figure.get_id()[..], location)
    }
}

