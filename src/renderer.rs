use std::collections::BTreeSet;
use std::collections::HashMap;
use crate::figures::Figure;
use geom_2d::point::Point;
use crate::{get_document, NAME_ID_PREFIX};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::errors::DomError::*;
use crate::errors::RendererError;
use crate::errors::RendererError::*;

const ROOT_NAME: &str = "root";

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
    pub fn new(dom_root_id: &str) -> Result<Renderer, RendererError> {
        let document = crate::get_document()?;

        let root = document
            .get_element_by_id(dom_root_id)
            .ok_or(Dom(UnfindableId(String::from(dom_root_id))))?;

        let svg_element = crate::create_element_ns("http://www.w3.org/2000/svg", "svg")?;
        let defs_element = crate::create_element_ns("http://www.w3.org/2000/svg", "defs")?;

        Renderer::set_view_box(&svg_element, 0, 0, 60, 60)?;

        svg_element.append_child(&defs_element)
            .map_err(|_| Dom(UnappendableElement))?;
        root.append_child(&svg_element)
            .map_err(|_| Dom(UnappendableElement))?;

        Ok(Renderer {
            dom_root_id: String::from(dom_root_id),
            shape_defs: BTreeSet::new(),
            name_defs: HashMap::new()
        })
    }

    fn set_view_box(element: &web_sys::Element, x: u32, y: u32, width: u32, height: u32) -> Result<(), RendererError> {
        let value = &format!("{} {} {} {}", x, y, width, height)[..];

        element
            .set_attribute("viewBox", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("viewBox"), String::from(value))))
    }

    pub fn clear(&mut self) {
        self.get_svg_root()
            .expect("Can't find SVG Root")
            .set_inner_html("");

        self.shape_defs = BTreeSet::new();
        self.name_defs = HashMap::new();
    }

    fn get_root(&self) -> Result<web_sys::Element, RendererError> {
        let document = crate::get_document()?;

        document
            .get_element_by_id(&self.dom_root_id[..])
            .ok_or(Dom(UnfindableId(self.dom_root_id.clone())))
    }

    fn get_svg_root(&self) -> Result<web_sys::Element, RendererError> {
        let root = self.get_root()?
            .first_element_child()
            .ok_or(Dom(EmptyContainer))?;

        if root.tag_name() != "svg" {
            return Err(Dom(UnfindableTag(String::from("svg"))))
        }

        Ok(root)
    }

    fn get_defs_root(&self) -> Result<web_sys::Element, RendererError> {
        let defs = self.get_svg_root()?
            .first_element_child();

        match defs {
            None => Err(Dom(EmptyContainer)),
            Some(root) => {
                if root.tag_name() != "defs" {
                    return Err(Dom(UnfindableTag(String::from("defs"))))
                }

                Ok(root)
            }
        }
    }

    fn contains_shape(&self, shape: &Figure) -> bool {
        let hash = shape.get_hash();

        self.shape_defs.contains(&hash)
    }

    fn add_def(&mut self, shape: &Figure) -> Result<(), RendererError> {
        self.get_defs_root()?
            .append_child(&shape.to_def())
            .map_err(|_| Dom(UnappendableElement))?;

        let hash = shape.get_hash();
        self.shape_defs.insert(hash);

        Ok(())
    }

    fn create_use(&self, def_id: &str, location: &Point) -> Result<web_sys::Element, RendererError> {
        let use_element = crate::create_element_ns(crate::SVG_NS, "use")?;

        let value = &format!("#{}", def_id)[..];
        use_element
            .set_attribute("href", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("href"), String::from(value))))?;

        let value = &format!("{}", location.x())[..];
        use_element
            .set_attribute("x", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("x"), String::from(value))))?;

        let value = &format!("{}", location.y())[..];
        use_element
            .set_attribute("y", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("y"), String::from(value))))?;

        Ok(use_element)
    }

    fn add_use(&self, def_id: &str, location: &Point) -> Result<(), RendererError> {
        let root = self.get_svg_root()?;
        let use_element = self.create_use(def_id, location)?;

        root
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    fn create_id_string(&mut self, name: &str) -> Result<String, RendererError> {
        if name == ROOT_NAME {
            return Err(Dom(IdAlreadyExists(String::from(ROOT_NAME))));
        }

        if self.name_defs.contains_key(name) {
            return Err(UnfindableName(String::from(name)));
        }

        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        let id_hash = s.finish();

        let id_string = Renderer::get_id_of_named(&id_hash);

        let element = get_document()?
            .get_element_by_id(&id_string[..]);
        match element {
            Some(_) => Err(Dom(IdAlreadyExists(id_string))),
            None => {
                self.name_defs.insert(String::from(name), id_hash);

                Ok(id_string)
            }
        }
    }

    fn get_id_of_named(id_hash: &u64) -> String {
        format!("{}-{:x}", NAME_ID_PREFIX, id_hash)
    }

    fn add_named_use(&mut self, name: &str, def_id: &str, location: &Point) -> Result<String, RendererError> {
        let id_string = self.create_id_string(name)?;

        let root = self.get_svg_root()?;
        let use_element = self.create_use(def_id, location)?;
        use_element.set_id(&id_string[..]);

        root
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))?;

        Ok(id_string)
    }

    fn get_named_container(&self, name: &str) -> Result<web_sys::Element, RendererError> {
        if name == ROOT_NAME {
            return self.get_svg_root();
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            return Err(UnfindableName(String::from(name)));
        }

        let id_hash = id_hash.unwrap();

        let container = get_document()?
            .get_element_by_id(
                &Renderer::get_id_of_named(id_hash)[..]
            ).ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        if container.tag_name() != "g" {
            return Err(NamedNotContainer(String::from(name)))
        }

        Ok(container)
    }

    fn add_use_to(&mut self, name: &str, def_id: &str, location: &Point) -> Result<(), RendererError> {
        let container = self.get_named_container(name)?;
        let use_element = self.create_use(def_id, location)?;

        container
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    fn adjust_use_to(&mut self, name: &str, def_id: &str, location: &Point) -> Result<(), RendererError> {
        if name == ROOT_NAME {
            return Err(NamedNotUse(String::from(ROOT_NAME)))
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            return Err(UnfindableName(String::from(name)));
        }

        let id_hash = id_hash.unwrap();

        let use_element = get_document()?
            .get_element_by_id(
                &Renderer::get_id_of_named(id_hash)[..]
            ).ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        if use_element.tag_name() != "use" {
            return Err(NamedNotUse(String::from(name)));
        }

        let value = &format!("#{}", def_id)[..];
        use_element
            .set_attribute("href", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("href"), String::from(value))))?;

        let value = &format!("{}", location.x())[..];
        use_element
            .set_attribute("x", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("x"), String::from(value))))?;

        let value = &format!("{}", location.y())[..];
        use_element
            .set_attribute("y", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("y"), String::from(value))))?;

        Ok(())
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
    pub fn render(&mut self, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add definition!");
        }

        // Add use of definition
        self.add_use(&figure.get_id()[..], location)
            .expect("Failed to add use!");
    }


    pub fn render_named(&mut self, name: &str, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add definition!");
        }

        // Add named use of definition
        self.add_named_use(name, &figure.get_id()[..], location)
            .expect("Failed to add named use!");
    }

    pub fn clear_named_container(&self, container_name: &str) {
        self.get_named_container(container_name)
            .expect("Failed to fetch named container!")
            .set_inner_html("");
    }

    pub fn update_named(&mut self, name: &str, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_shape(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add named definition!");
        }


        let container = self.get_named_container(name);

        if !container.is_err() {
            // Delete all current elements in de container
            self.clear_named_container(name);

            // Add element to container
            self.add_use_to(name, &figure.get_id(), location)
                .expect("Failed to add named use!");
        } else {
            // Adjust use element
            self.adjust_use_to(name, &figure.get_id(), location)
                .expect("Failed to adjust use element!");
        }
    }

    pub fn hide_named(&self, name: &str) {
        self.get_named_container(name)
            .expect("Failed to fetch named container!")
            .set_attribute("style", "display: none;")
            .expect("Failed to set attribute of container!");
    }

    pub fn show_named(&self, name: &str) {
        self.get_named_container(name)
            .expect("Failed to fetch named container!")
            .remove_attribute("style")
            .expect("Failed to set attribute of container!");
    }

    /// Deletes a named child of the svg, and **all** its children.
    pub fn delete_named(&mut self, name: &str) {
        let container = self.get_named_container(name)
            .expect("Failed to fetch named container!");

        let parent = container.parent_element()
            .ok_or(NoParent)
            .expect("No parent was found!");

        parent.remove_child(&container)
            .map_err(|_| Dom(UnremoveableChild))
            .expect("Failed to remove child!");
    }

    pub fn does_name_exist(&self, name: &str) -> bool {
        self.name_defs.contains_key(name)
    }

    pub fn create_named_container(&mut self, name: &str, parent: &str) {
        let parent = self.get_named_container(parent)
            .expect("Failed to fetch named container!");

        let id_string = self.create_id_string(name)
            .expect("Unable to create id string");

        let container = crate::create_element_ns("http://www.w3.org/2000/svg", "g")
            .expect("Failed to create new named container!");
        container.set_id(&id_string[..]);

        parent.append_child(&container)
            .map_err(|_| Dom(UnappendableElement))
            .expect("Failed to append named container to parent!");
    }
}

