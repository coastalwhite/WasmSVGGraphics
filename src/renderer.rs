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

/// The value to which the view box of every svg [Renderer](struct.Renderer.html) will be set to by default.
/// The viewbox of any [Renderer](struct.Renderer.html),
/// can be adjusted with the [adjust_viewbox](struct.Renderer.html#method.adjust_viewbox) method.
///
/// # Form
/// The DEFAULT_VIEWBOX constant is given in the following form:
/// `[x, y, width, height]`
pub const DEFAULT_VIEWBOX: [i32; 4] = [0, 0, 100, 100];

/// Container object used to interact with the SVG Object
/// Keeps track of definitions and dom root id
pub struct Renderer {
    /// The id of the SVG element within the dom
    dom_root_id: String,

    /// All the already defined SVG definitions
    figure_defs: BTreeSet<u64>,

    /// All the names in use
    name_defs: HashMap<String, u64>
}

impl Renderer {
    /// sets the viewbox
    fn set_view_box(element: &web_sys::Element, x: i32, y: i32, width: i32, height: i32) -> Result<(), RendererError> {
        let value = &format!("{} {} {} {}", x, y, width, height)[..];

        element
            .set_attribute("viewBox", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("viewBox"), String::from(value))))
    }

    /// Will return the parent of the svg
    fn get_root(&self) -> Result<web_sys::Element, RendererError> {
        let document = crate::get_document()?;

        document
            .get_element_by_id(&self.dom_root_id[..])
            .ok_or(Dom(UnfindableId(self.dom_root_id.clone())))
    }

    /// Will return the svg root
    fn get_svg_root(&self) -> Result<web_sys::Element, RendererError> {
        let root = self.get_root()?
            .first_element_child()
            .ok_or(Dom(EmptyContainer))?;

        if root.tag_name() != "svg" {
            return Err(Dom(UnfindableTag(String::from("svg"))))
        }

        Ok(root)
    }

    /// Will return the defs element
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

    /// Returns whether the renderer already has a definition for the shape
    fn contains_figure(&self, figure: &Figure) -> bool {
        self.contains_id(figure.get_hash())
    }

    /// Returns whether the renderer already has a definition for the shape
    fn contains_id(&self, figure_id: u64) -> bool {
        self.figure_defs.contains(&figure_id)
    }

    /// Adds a def to the binary tree
    fn add_def(&mut self, figure: &Figure) -> Result<(), RendererError> {
        self.get_defs_root()?
            .append_child(&figure.to_def())
            .map_err(|_| Dom(UnappendableElement))?;

        let hash = figure.get_hash();
        self.figure_defs.insert(hash);

        Ok(())
    }

    /// Creates a use element from a def_id and location
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

    /// Creates a new id string from name
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

    /// Will convert a id hash into a HTML id Attribute
    fn get_id_of_named(id_hash: &u64) -> String {
        format!("{}-{:x}", NAME_ID_PREFIX, id_hash)
    }

    fn get_id_of_figure(id_hash: u64) -> String {
        format!("{}-{}", super::SHAPE_ID_PREFIX, format!("{:x}", id_hash))
    }

    /// Will retrieve the web_sys element of a named container
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

    /// Will add a use element to the root svg
    fn add_use(&self, def_id: &str, location: &Point) -> Result<(), RendererError> {
        let root = self.get_svg_root()?;
        let use_element = self.create_use(def_id, location)?;

        root
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    /// Will add a use element to the root svg with a name
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

    /// Will add a use element to a named container
    fn add_use_to(&mut self, name: &str, def_id: &str, location: &Point) -> Result<(), RendererError> {
        let container = self.get_named_container(name)?;
        let use_element = self.create_use(def_id, location)?;

        container
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    /// Adjust a named use to another figure
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

    /// Deletes a use element
    fn delete_use(&mut self, name: &str) -> Result<(), RendererError> {
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

        let parent = use_element
            .parent_element()
            .ok_or(Dom(NoParent))?;

        parent.remove_child(&use_element)
            .map_err(|_| Dom(UnremoveableChild))?;

        Ok(())
    }

    /// Create new renderer object
    ///
    /// # Arguments
    /// * `dom_root_id` - The HTMl Attribute ID of the parent element of to be created SVG
    pub fn new(dom_root_id: &str) -> Result<Renderer, RendererError> {
        let document = crate::get_document()?;

        let root = document
            .get_element_by_id(dom_root_id)
            .ok_or(Dom(UnfindableId(String::from(dom_root_id))))?;

        let svg_element = crate::create_element_ns("http://www.w3.org/2000/svg", "svg")?;
        let defs_element = crate::create_element_ns("http://www.w3.org/2000/svg", "defs")?;

        Renderer::set_view_box(
            &svg_element,
            DEFAULT_VIEWBOX[0],
            DEFAULT_VIEWBOX[1],
            DEFAULT_VIEWBOX[2],
            DEFAULT_VIEWBOX[3]
        )?;

        svg_element.append_child(&defs_element)
            .map_err(|_| Dom(UnappendableElement))?;
        root.append_child(&svg_element)
            .map_err(|_| Dom(UnappendableElement))?;

        Ok(Renderer {
            dom_root_id: String::from(dom_root_id),
            figure_defs: BTreeSet::new(),
            name_defs: HashMap::new()
        })
    }

    /// Render figure at a location (this will automatically add a definition when needed)
    ///
    /// # Arguments
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render(&circle, &Point::new(20, 20));
    /// ```
    pub fn render(&mut self, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_figure(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add definition!");
        }

        // Add use of definition
        self.add_use(&figure.get_id()[..], location)
            .expect("Failed to add use!");
    }

    /// Render a named figure at a location (this will automatically add a definition when needed)
    /// One can use named-figures for later adjustments, e.g. updating the location of the figure, hiding/showing the figure
    ///
    /// # Arguments
    /// * `name` - Name to use for later reference
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Panics
    /// Cannot create duplicate names or declare a name more than once.
    /// When an attempt is made to declare a name more than once, a panic will occur.
    /// One is able to check if a name already exists with the [does_name_exist](#method.does_name_exist) method.
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // --snip
    ///
    /// // Updates the named figure's location to (20,20)
    /// renderer.update_named("named_circle", &circle, &Point::new(20, 20));
    /// ```
    pub fn render_named(&mut self, name: &str, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_figure(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add definition!");
        }

        // Add named use of definition
        self.add_named_use(name, &figure.get_id()[..], location)
            .expect("Failed to add named use!");
    }

    /// Render figure from a previously added definition at a location (this will automatically add a definition when needed)
    ///
    /// # Arguments
    /// * `figure_id` - 8 byte hash of the figure used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// let circle_id = renderer.define_render(&circle);
    ///
    /// // Render circle
    /// renderer.render_id(circle_id, &Point::new(20, 20));
    /// ```
    pub fn render_id(&mut self, figure_id: u64, location: &Point) {
        // If there is already a definition
        if !self.contains_id(figure_id) {

            // Add the definition to the dom and hashes
            panic!("Definition doesn't exist");
        }

        // Add use of definition
        self.add_use(&Renderer::get_id_of_figure(&figure_id)[..], location)
            .expect("Failed to add use from id!");
    }


    /// Render figure from a previously added definition at a location (this will automatically add a definition when needed)
    ///
    /// # Arguments
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Define the render
    /// let circle_id = renderer.define_render(&circle);
    ///
    /// // Render circle
    /// renderer.render_id(circle_id, &Point::new(20, 20));
    /// ```
    pub fn define_render(&mut self, figure: &Figure) -> u64 {
        // If there is already a definition
        if !self.contains_figure(figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add definition!");
        }

        figure.get_hash()
    }


    /// Clears all elements within the SVG element and clears all internal definitions.
    /// Basically reinits the renderer.
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // --snip
    ///
    /// // Reinit the Renderer object
    /// renderer.clear();
    ///
    /// // Renders the circle with "named_circle" name again.
    /// // This would normally panic, since a name is redeclared,
    /// // but since the renderer is cleared, it will not. :)
    /// renderer.render_named("named_circle", &circle, &Point::new(20, 20));
    /// ```
    pub fn clear(&mut self) {
        self.get_svg_root()
            .expect("Can't find SVG Root")
            .set_inner_html("");

        self.figure_defs = BTreeSet::new();
        self.name_defs = HashMap::new();
    }

    /// Clears all figures/containers within a named container, but does not clear up definitions.
    ///
    /// # Arguments
    /// * `container_name` - The name of the container to clear
    ///
    ///
    /// # Panics
    /// Will panic when a name passed in is in use by a pure figure.
    /// For pure figures use [delete_named](#method.delete_named) or [hide_named](#method.hide_named) instead.
    ///
    /// # Note
    /// Most of the time the [update_named](#method.update_named) method is a better alternative.
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", &circle, &Point::new(10, 10));
    ///
    /// // Now the container contains the circle figure
    ///
    /// // --snip
    ///
    /// // Clear the named container
    /// renderer.clear_named_container("named_container");
    ///
    /// // Now the container is empty again
    ///
    /// // Render circle in the named_container.
    /// // Since definitions were not cleared, it will use a previous definition.
    /// // This saves some processing time in this case.
    /// renderer.append_to_container("named_container", &circle, &Point::new(20, 20));
    ///
    /// // Now the container contains the circle at a different position
    /// ```
    pub fn clear_named_container(&self, container_name: &str) {
        self.get_named_container(container_name)
            .expect("Failed to fetch named container!")
            .set_inner_html("");
    }

    /// Updates a named container or figure to either contain the passed figure or become the passed figure, respectively.
    ///
    /// # Arguments
    /// * `name` - The name of either a named container or a named figure
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", &circle, &Point::new(10, 10));
    ///
    /// // Now the container contains the circle figure
    ///
    /// // --snip
    ///
    /// // Update the contents of the named container
    /// renderer.update_named("named_container", &circle, &Point::new(20, 20));
    ///
    /// // Now the container contains the circle at a different position
    /// ```
    pub fn update_named(&mut self, name: &str, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_figure(figure) {

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

    /// Hides a named item in the DOM, this can be undone by the [show_named](#method.show_named) method.
    ///
    /// # Arguments
    /// * `name` - Name of item to hide
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // --snip
    ///
    /// // Hides the named figure
    /// renderer.hide_named("named_circle");
    /// ```
    pub fn hide_named(&self, name: &str) {
        self.get_named_container(name)
            .expect("Failed to fetch named container!")
            .set_attribute("style", "display: none;")
            .expect("Failed to set attribute of container!");
    }

    /// Shows a named item in the DOM, this undoes the [hide_named](#method.hide_named) method.
    ///
    /// # Arguments
    /// * `name` - Name of item to show
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // Hides the named figure
    /// renderer.hide_named("named_circle");
    ///
    /// // --snip
    ///
    /// // Show the named figure again
    /// renderer.show_named("named_circle");
    /// ```
    pub fn show_named(&self, name: &str) {
        self.get_named_container(name)
            .expect("Failed to fetch named container!")
            .remove_attribute("style")
            .expect("Failed to set attribute of container!");
    }

    /// Appends a figure to a named container
    ///
    /// # Arguments
    /// * `name` - The name of either a named container
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Panics
    /// Will panic when a name passed in is in use by a pure figure.
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", &circle, &Point::new(10, 10));
    ///
    /// // Now the container contains the circle figure
    /// ```
    pub fn append_to_container(&mut self, name: &str, figure: &Figure, location: &Point) {
        // If there is already a definition
        if !self.contains_figure(figure) {

            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add named definition!");
        }

        self.add_use_to(name, &figure.get_id()[..], &location)
            .expect("Failed to add figure to container!")
    }

    /// Deletes a named item from the DOM and from internal entries.
    /// But will not delete definitions
    ///
    /// # Arguments
    /// * `name` - Name of item to delete
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // --snip
    ///
    /// // Delete the named figure
    /// renderer.delete_named("named_circle");
    ///
    /// // Renders the circle with "named_circle" name again.
    /// // This would normally panic, since a name is redeclared,
    /// // but since the named figure is deleted, it will not. :)
    /// renderer.render_named("named_circle", &circle, &Point::new(20, 20));
    /// ```
    pub fn delete_named(&mut self, name: &str) {
        let container = self.get_named_container(name);

        if !container.is_err() {
            let container = container.unwrap();

            let parent = container.parent_element()
                .ok_or(NoParent)
                .expect("No parent was found!");

            parent.remove_child(&container)
                .map_err(|_| Dom(UnremoveableChild))
                .expect("Failed to remove child!");
        } else {
            // Adjust use element
            self.delete_use(name)
                .expect("Failed to delete use element!");
        }

        self.name_defs.remove(name);
    }

    /// Will return if a certain name exists and therefore cannot be used for a declaration.
    ///
    /// # Arguments
    /// * 'name' - Name to check
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Will be set to false
    /// let does_named_circle_exist = renderer.does_name_exist("named_circle");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // Will be set to true
    /// let does_named_circle_exist = renderer.does_name_exist("named_circle");
    ///
    /// // Delete the named figure
    /// renderer.delete_named("named_circle");
    ///
    /// // Will be set to false
    /// let does_named_circle_exist = renderer.does_name_exist("named_circle");
    ///
    /// // Renders the circle with "named_circle" name again.
    /// // This would normally panic, since a name is redeclared,
    /// // but since the named figure is deleted, it will not. :)
    /// renderer.render_named("named_circle", &circle, &Point::new(20, 20));
    ///
    /// // Will be set to true
    /// let does_named_circle_exist = renderer.does_name_exist("named_circle");
    /// ```
    pub fn does_name_exist(&self, name: &str) -> bool {
        self.name_defs.contains_key(name)
    }

    /// Creates a new named container in the parent
    ///
    /// # Arguments
    /// * `name` - Name of the named container used for later reference
    /// * `parent` - Name a container, which to use as parent ("root" is used for the SVG root)
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", &circle, &Point::new(10, 10));
    /// ```
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

    /// Will return whether a given name is used for a named container, instead of a pure figure
    ///
    /// # Arguments
    /// * `name` - Name to check
    ///
    /// # Note
    /// Will output false if the name is not in use
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = figures::preset::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", &circle, &Point::new(10, 10));
    ///
    /// // Create a named container
    /// renderer.create_named_container("named_container", "root");
    ///
    /// println!("{}", renderer.is_container("named_circle")); // false
    /// println!("{}", renderer.is_container("named_container")); // true
    /// println!("{}", renderer.is_container("not_in_use_name")); // false
    /// ```
    pub fn is_container(&self, name: &str) -> bool {
        self.get_named_container(name).is_ok()
    }

    /// Adjusts the viewbox of the svg
    ///
    /// # Arguments
    /// * `x` - The top-left x-coordinate of the viewbox
    /// * `y` - The top-left y-coordinate of the viewbox
    /// * `width` - The width of the viewbox
    /// * `height` - The height of the viewbox
    ///
    /// # Note
    /// By default this is set to [DEFAULT_VIEWBOX](constant.DEFAULT_VIEWBOX.html).
    ///
    /// # Examples
    /// ```
    /// use wasm_svg_graphics::figures;
    /// use geom_2d::point::Point;
    /// use wasm_svg_graphics::renderer::Renderer;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = Renderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adjust the viewbox
    /// renderer.adjust_viewbox(0, 0, 50, 50);
    /// ```
    pub fn adjust_viewbox(&self, x: i32, y: i32, width: i32, height: i32) {
        Renderer::set_view_box(
            &self.get_svg_root()
                .expect("Failed to retrieve SVG container!"),
            x,
            y,
            width,
            height
        ).expect("Failed to set viewBox!");
    }
}

