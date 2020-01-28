use std::collections::BTreeSet;
use std::collections::HashMap;
use crate::figures::Figure;
use geom_2d::point::Point;
use crate::{get_document, NAME_ID_PREFIX};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Container object used to interact with the SVG Object
/// Keeps track of definitions and dom root id
pub struct Renderer {
    /// The id of the SVG element within the dom
    dom_root_id: String,

    /// All the already defined SVG definitions
    shape_defs: BTreeSet<u64>,

    /// All the names in use
    name_defs: HashMap<String, u64>
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

        Renderer::setViewBox(&svg_element, 0, 0, 60, 60);

        svg_element.append_child(&defs_element).expect("Unable to append defs to svg");
        root.append_child(&svg_element).expect("Unable to create svg to root");

        Renderer {
            dom_root_id: String::from(dom_root_id),
            shape_defs: BTreeSet::new(),
            name_defs: HashMap::new()
        }
    }

    fn setViewBox(element: &web_sys::Element, x: u32, y: u32, width: u32, height: u32) {
        element
            .set_attribute("viewBox", &format!("{} {} {} {}", x, y, width, height)[..])
            .expect("Couldn't set viewBox of svg element");
    }

    pub fn clear(&mut self) {
        self.get_svg_root().set_inner_html("");

        self.shape_defs = BTreeSet::new();
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

    fn create_use(&self, def_id: &str, location: Point) -> web_sys::Element {
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

        use_element
    }

    fn add_use(&self, def_id: &str, location: Point) {
        let root = self.get_svg_root();
        let use_element = self.create_use(def_id, location);

        root
            .append_child(&use_element)
            .expect("Unable to add use of Figure");
    }

    fn create_id_string(&mut self, name: &str) -> Option<String> {
        if name == "root" {
            return None;
        }

        if self.name_defs.contains_key(name) {
            return None;
        }

        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        let id_hash = s.finish();

        let id_string = Renderer::get_id_of_named(&id_hash);

        let element = get_document().get_element_by_id(&id_string[..]);
        match element {
            Some(_) => None,
            None => {
                self.name_defs.insert(String::from(name), id_hash);

                Some(id_string)
            }
        }
    }

    fn get_id_of_named(id_hash: &u64) -> String {
        format!("{}-{:x}", NAME_ID_PREFIX, id_hash)
    }

    fn add_named_use(&mut self, name: &str, def_id: &str, location: Point) {
        let id_string = self.create_id_string(name);

        if id_string == None {
            panic!("Can't create new item with this name");
        }

        let id_string = id_string.unwrap();

        let root = self.get_svg_root();
        let use_element = self.create_use(def_id, location);
        use_element.set_id(&id_string[..]);

        root
            .append_child(&use_element)
            .expect("Unable to add use of Figure");
    }

    fn get_named_container(&self, name: &str) -> web_sys::Element {
        if name == "root" {
            return self.get_svg_root();
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            panic!("Can't find name item");
        }

        let id_hash = id_hash.unwrap();

        let container = get_document()
            .get_element_by_id(
                &Renderer::get_id_of_named(id_hash)[..]
            );

        match container {
            None => panic!("Container doesn't exist in dom"),
            Some(container) => {
                if container.tag_name() != "g" {
                    panic!("Named element is not a container");
                }

                container
            }
        }
    }

    fn add_use_to(&mut self, name: &str, def_id: &str, location: Point) {
        let container = self.get_named_container(name);
        let use_element = self.create_use(def_id, location);

        container
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
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = wasm_svg_graphics::renderer::Renderer::new("svg_id");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render(&circle, Point::new(20, 20));
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


    pub fn render_named(&mut self, name: &str, figure: &Figure, location: Point) {
        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure);
        }

        // Add named use of definition
        self.add_named_use(name, &figure.get_id()[..], location)
    }

    pub fn clear_named(&self, container_name: &str) {
        self.get_named_container(container_name).set_inner_html("");
    }

    pub fn update_named(&mut self, container_name: &str, figure: &Figure, location: Point) {
        // Delete all current elements in de container
        self.clear_named(container_name);

        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure);
        }

        // Add element to container
        self.add_use_to(container_name, &figure.get_id(), location);
    }

    pub fn hide_named(&self, name: &str) {
        self.get_named_container(name).set_attribute("style", "display: none;");
    }

    pub fn show_named(&self, name: &str) {
        self.get_named_container(name).remove_attribute("style");
    }

    /// Deletes a named child of the svg, and **all** its children.
    pub fn delete_named(&mut self, name: &str) {
        let container = self.get_named_container(name);

        let parent = container.parent_element();

        match parent {
            None => panic!("No parent was found!"),
            Some(parent) => parent.remove_child(&container)
        };
    }

    pub fn does_name_exist(&self, name: &str) -> bool {
        self.name_defs.contains_key(name)
    }

    pub fn create_named_container(&mut self, name: &str, parent: &str) {
        let parent = self.get_named_container(parent);

        let id_string = self.create_id_string(name);

        if id_string == None {
            panic!("Unable to create id_string");
        }

        let id_string = id_string.unwrap();

        let container = crate::create_element_ns("http://www.w3.org/2000/svg", "g");
        container.set_id(&id_string[..]);

        parent.append_child(&container);
    }
}

