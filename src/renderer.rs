//! Renderer of SVG Graphics within the webpage, contains definitions and names

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use svg_definitions::prelude::*;

use crate::errors::DomError::*;
use crate::errors::RendererError;
use crate::errors::RendererError::*;
use crate::{get_document, NAME_ID_PREFIX};

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
    name_defs: HashMap<String, u64>,
}

impl Renderer {
    /// sets the viewbox
    fn set_view_box(
        element: &web_sys::Element,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), RendererError> {
        let value = &format!("{} {} {} {}", x, y, width, height)[..];

        element.set_attribute("viewBox", value).map_err(|_| {
            Dom(UnsetableAttribute(
                String::from("viewBox"),
                String::from(value),
            ))
        })
    }

    fn get_hash(figure: &SVGElem) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        figure.hash(&mut hasher);

        hasher.finish()
    }

    fn to_def(figure: SVGElem) -> web_sys::Element {
        let elem = crate::to_html(&figure);
        elem.set_id(&Self::get_id_of_figure(Self::get_hash(&figure)));
        elem
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
        let root = self
            .get_root()?
            .first_element_child()
            .ok_or(Dom(EmptyContainer))?;

        if root.tag_name() != "svg" {
            return Err(Dom(UnfindableTag(String::from("svg"))));
        }

        Ok(root)
    }

    /// Will return the defs element
    fn get_defs_root(&self) -> Result<web_sys::Element, RendererError> {
        let defs = self.get_svg_root()?.first_element_child();

        match defs {
            None => Err(Dom(EmptyContainer)),
            Some(root) => {
                if root.tag_name() != "defs" {
                    return Err(Dom(UnfindableTag(String::from("defs"))));
                }

                Ok(root)
            }
        }
    }

    /// Returns whether the renderer already has a definition for the shape
    fn contains_figure(&self, figure: &SVGElem) -> bool {
        self.contains_id(Self::get_hash(figure))
    }

    /// Returns whether the renderer already has a definition for the shape
    fn contains_id(&self, figure_id: u64) -> bool {
        self.figure_defs.contains(&figure_id)
    }

    /// Adds a def to the binary tree
    fn add_def(&mut self, figure: SVGElem) -> Result<(), RendererError> {
        let hash = Self::get_hash(&figure);

        self.get_defs_root()?
            .append_child(&web_sys::Node::from(Self::to_def(figure)))
            .map_err(|_| Dom(UnappendableElement))?;

        self.figure_defs.insert(hash);

        Ok(())
    }

    /// Creates a use element from a def_id and location
    fn create_use(
        &self,
        def_id: &str,
        location: Point2D,
    ) -> Result<web_sys::Element, RendererError> {
        Ok(crate::to_html(
            &SVGElem::new(Tag::Use)
                .set(Attr::PositionX, location.0.into())
                .set(Attr::PositionY, location.1.into())
                .set(
                    Attr::Reference,
                    AttrValue::new_reference(def_id).expect("Invalid href id"),
                ),
        ))
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

        let element = get_document()?.get_element_by_id(&id_string[..]);
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
            .get_element_by_id(&Renderer::get_id_of_named(id_hash)[..])
            .ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        if container.tag_name() != "g" {
            return Err(NamedNotContainer(String::from(name)));
        }

        Ok(container)
    }

    /// Will retrieve the web_sys element of a named container
    fn get_named_item(&self, name: &str) -> Result<web_sys::Element, RendererError> {
        if name == ROOT_NAME {
            return self.get_svg_root();
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            return Err(UnfindableName(String::from(name)));
        }

        let id_hash = id_hash.unwrap();

        let container = get_document()?
            .get_element_by_id(&Renderer::get_id_of_named(id_hash)[..])
            .ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        Ok(container)
    }

    /// Will add a use element to the root svg
    fn add_use(&self, def_id: &str, location: Point2D) -> Result<(), RendererError> {
        let root = self.get_svg_root()?;
        let use_element = self.create_use(def_id, location)?;

        root.append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    /// Will add a use element to the root svg with a name
    fn add_named_use(
        &mut self,
        name: &str,
        def_id: &str,
        location: Point2D,
    ) -> Result<String, RendererError> {
        let id_string = self.create_id_string(name)?;

        let root = self.get_svg_root()?;
        let use_element = self.create_use(def_id, location)?;
        use_element.set_id(&id_string[..]);

        root.append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))?;

        Ok(id_string)
    }

    /// Will add a use element to a named container
    fn add_use_to(
        &mut self,
        name: &str,
        def_id: &str,
        location: Point2D,
    ) -> Result<(), RendererError> {
        let container = self.get_named_container(name)?;
        let use_element = self.create_use(def_id, location)?;

        container
            .append_child(&use_element)
            .map_err(|_| Dom(UnappendableElement))
            .map(|_| ())
    }

    /// Adjust a named use to another figure
    fn adjust_use_to(
        &mut self,
        name: &str,
        def_id: &str,
        location: Point2D,
    ) -> Result<(), RendererError> {
        if name == ROOT_NAME {
            return Err(NamedNotUse(String::from(ROOT_NAME)));
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            return Err(UnfindableName(String::from(name)));
        }

        let id_hash = id_hash.unwrap();

        let use_element = get_document()?
            .get_element_by_id(&Renderer::get_id_of_named(id_hash)[..])
            .ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        if use_element.tag_name() != "use" {
            return Err(NamedNotUse(String::from(name)));
        }

        let value = &format!("#{}", def_id)[..];
        use_element.set_attribute("href", value).map_err(|_| {
            Dom(UnsetableAttribute(
                String::from("href"),
                String::from(value),
            ))
        })?;

        let value = &format!("{:.2}", location.0)[..];
        use_element
            .set_attribute("x", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("x"), String::from(value))))?;

        let value = &format!("{:.2}", location.1)[..];
        use_element
            .set_attribute("y", value)
            .map_err(|_| Dom(UnsetableAttribute(String::from("y"), String::from(value))))?;

        Ok(())
    }

    /// Deletes a use element
    fn delete_use(&mut self, name: &str) -> Result<(), RendererError> {
        if name == ROOT_NAME {
            return Err(NamedNotUse(String::from(ROOT_NAME)));
        }

        let id_hash = self.name_defs.get(name);

        if id_hash == None {
            return Err(UnfindableName(String::from(name)));
        }

        let id_hash = id_hash.unwrap();

        let use_element = get_document()?
            .get_element_by_id(&Renderer::get_id_of_named(id_hash)[..])
            .ok_or(Dom(UnfindableId(Renderer::get_id_of_named(id_hash))))?;

        if use_element.tag_name() != "use" {
            return Err(NamedNotUse(String::from(name)));
        }

        let parent = use_element.parent_element().ok_or(Dom(NoParent))?;

        parent
            .remove_child(&use_element)
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

        let svg_element = SVGElem::new(Tag::SVG)
            .set(
                Attr::ViewBox,
                (
                    DEFAULT_VIEWBOX[0],
                    DEFAULT_VIEWBOX[1],
                    DEFAULT_VIEWBOX[2],
                    DEFAULT_VIEWBOX[3],
                )
                    .into(),
            )
            .append(SVGElem::new(Tag::Defs));

        root.append_child(&crate::to_html(&svg_element))
            .map_err(|_| Dom(UnappendableElement))?;

        Ok(Renderer {
            dom_root_id: String::from(dom_root_id),
            figure_defs: BTreeSet::new(),
            name_defs: HashMap::new(),
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render(circle, (20.0, 20.0));
    /// ```
    pub fn render(&mut self, figure: SVGElem, location: Point2D) {
        let figure_id = Self::get_id_of_figure(Self::get_hash(&figure));

        // If there is already a definition
        if !self.contains_figure(&figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure).expect("Failed to add definition!");
        }

        // Add use of definition
        self.add_use(&figure_id[..], location)
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", circle, (10.0, 10.0));
    ///
    /// // --snip
    ///
    /// // Updates the named figure's location to (20,20)
    /// renderer.move_named("named_circle", (20.0, 20.0));
    /// ```
    pub fn render_named(&mut self, name: &str, figure: SVGElem, location: Point2D) {
        let figure_id = Self::get_id_of_figure(Self::get_hash(&figure));

        // If there is already a definition
        if !self.contains_figure(&figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure).expect("Failed to add definition!");
        }

        // Add named use of definition
        self.add_named_use(name, &figure_id[..], location)
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// let circle_id = renderer.define_render(circle);
    ///
    /// // Render circle
    /// renderer.render_id(circle_id, (20.0, 20.0));
    /// ```
    pub fn render_id(&mut self, figure_id: u64, location: Point2D) {
        // If there is already a definition
        if !self.contains_id(figure_id) {
            // Add the definition to the dom and hashes
            panic!("Definition doesn't exist");
        }

        // Add use of definition
        self.add_use(&Renderer::get_id_of_figure(figure_id)[..], location)
            .expect("Failed to add use from id!");
    }

    /// Render named figure from a previously added definition at a location (this will automatically add a definition when needed)
    ///
    /// # Arguments
    /// * `name` - Name to use for later reference
    /// * `figure_id` - 8 byte hash of the figure used when adding to the dom
    /// * `location` - the location where to add the `figure`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// let circle_id = renderer.define_render(circle);
    ///
    /// // Render circle
    /// renderer.render_named_id("named_circle", circle_id, (20.0, 20.0));
    ///
    /// // --snip
    ///
    /// // Updates the Circle's location
    /// renderer.move_named("named_circle", (25.0, 25.0));
    /// ```
    pub fn render_named_id(&mut self, name: &str, figure_id: u64, location: Point2D) {
        // If there is already a definition
        if !self.contains_id(figure_id) {
            // Add the definition to the dom and hashes
            panic!("Definition doesn't exist");
        }

        // Add use of definition
        self.add_named_use(name, &Renderer::get_id_of_figure(figure_id)[..], location)
            .expect("Failed to add named use from id!");
    }

    /// Define a figure and return it's hash, this hash can later be used for rendering
    ///
    /// # Arguments
    /// * `figure` - [Figure](../figures/struct.Figure.html) object, used when adding to the dom
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Define the render
    /// let circle_id = renderer.define_render(circle);
    ///
    /// // Render circle
    /// renderer.render_id(circle_id, (20.0, 20.0));
    /// ```
    pub fn define_render(&mut self, figure: SVGElem) -> u64 {
        let figure_hash = Self::get_hash(&figure);

        // If there is already a definition
        if !self.contains_figure(&figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure).expect("Failed to add definition!");
        }

        figure_hash
    }

    /// Clears all elements within the SVG element and clears all internal definitions.
    /// Basically reinits the renderer.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));
    ///
    /// // --snip
    ///
    /// // Reinit the Renderer object
    /// renderer.clear();
    ///
    /// // Renders the circle with "named_circle" name again.
    /// // This would normally panic, since a name is redeclared,
    /// // but since the renderer is cleared, it will not. :)
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));
    /// ```
    pub fn clear(&mut self) {
        self.get_svg_root()
            .expect("Can't find SVG Root")
            .set_inner_html("");

        self.get_svg_root()
            .expect("Can't find SVG Root (2)")
            .append_child(&crate::to_html(&SVGElem::new(Tag::Defs)))
            .expect("Failed to append defs!");

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
    /// Also have a look at [hide_named](#method.hide_named) and [move_named](#method.move_named)
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));
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
    /// renderer.append_to_container("named_container", SVGDefault::circle(10), (20.0, 20.0));
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));
    ///
    /// // Now the container contains the circle figure
    ///
    /// // --snip
    ///
    /// // Update the contents of the named container
    /// renderer.update_named("named_container", SVGDefault::circle(10), (20.0, 20.0));
    ///
    /// // Now the container contains the circle at a different position
    /// ```
    pub fn update_named(&mut self, name: &str, figure: SVGElem, location: Point2D) {
        let figure_id = Self::get_id_of_figure(Self::get_hash(&figure));

        // If there is already a definition
        if !self.contains_figure(&figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add named definition!");
        }

        let container = self.get_named_container(name);

        if !container.is_err() {
            // Delete all current elements in de container
            self.clear_named_container(name);

            // Add element to container
            self.add_use_to(name, &figure_id, location)
                .expect("Failed to add named use!");
        } else {
            // Adjust use element
            self.adjust_use_to(name, &figure_id, location)
                .expect("Failed to adjust use element!");
        }
    }

    /// Updates a named container or figure to either contain the passed figure
    /// or become the passed figure from the id, respectively.
    ///
    /// # Arguments
    /// * `name` - The name of either a named container or a named figure
    /// * `figure_id` - id of Figure definition used when adding to the dom,
    /// defined using [define_render](#method.define_render)
    /// * `location` - the location where to add the `figure`
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// let circle_id = renderer.define_render(SVGDefault::circle(10));
    ///
    /// // Render circle
    /// renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));
    ///
    /// // Now the container contains the circle figure
    ///
    /// // --snip
    ///
    /// // Update the contents of the named container
    /// renderer.update_named_with_id("named_container", circle_id, (20.0, 20.0));
    ///
    /// // Now the container contains the circle at a different position
    /// ```
    pub fn update_named_with_id(&mut self, name: &str, figure_id: u64, location: Point2D) {
        // If there is already a definition
        if !self.contains_id(figure_id) {
            panic!("No definition found!");
        }

        let container = self.get_named_container(name);

        if !container.is_err() {
            // Delete all current elements in de container
            self.clear_named_container(name);

            // Add element to container
            self.add_use_to(name, &Renderer::get_id_of_figure(figure_id)[..], location)
                .expect("Failed to add named use!");
        } else {
            // Adjust use element
            self.adjust_use_to(name, &Renderer::get_id_of_figure(figure_id)[..], location)
                .expect("Failed to adjust use element!");
        }
    }

    /// Hides a named item in the DOM, this can be undone by the [show_named](#method.show_named) method.
    ///
    /// # Arguments
    /// * `name` - Name of item to hide
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", circle, (10.0, 10.0));
    ///
    /// // --snip
    ///
    /// // Hides the named figure
    /// renderer.hide_named("named_circle");
    /// ```
    pub fn hide_named(&self, name: &str) {
        self.get_named_item(name)
            .expect("Failed to fetch named item!")
            .set_attribute("style", "display: none;")
            .expect("Failed to set attribute of container!");
    }

    /// Shows a named item in the DOM, this undoes the [hide_named](#method.hide_named) method.
    ///
    /// # Arguments
    /// * `name` - Name of item to show
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", circle, (10.0, 10.0));
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
        self.get_named_item(name)
            .expect("Failed to fetch named item!")
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", circle, (10.0, 10.0));
    ///
    /// // Now the container contains the circle figure
    /// ```
    pub fn append_to_container(&mut self, name: &str, figure: SVGElem, location: Point2D) {
        let figure_id = Self::get_id_of_figure(Self::get_hash(&figure));

        // If there is already a definition
        if !self.contains_figure(&figure) {
            // Add the definition to the dom and hashes
            self.add_def(figure)
                .expect("Failed to add named definition!");
        }

        self.add_use_to(name, &figure_id[..], location)
            .expect("Failed to add figure to container!")
    }

    /// Appends a figure from id to a named container
    ///
    /// # Arguments
    /// * `name` - The name of either a named container
    /// * `figure_id` - id of Figure definition used when adding to the dom,
    /// defined using [define_render](#method.define_render)
    /// * `location` - the location where to add the `figure`
    ///
    /// # Panics
    /// Will panic when a name passed in is in use by a pure figure.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// let circle_id = renderer.define_render(SVGDefault::circle(10));
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container_with_id("named_container", circle_id, (10.0, 10.0));
    ///
    /// // Now the container contains the circle figure
    /// ```
    pub fn append_to_container_with_id(&mut self, name: &str, figure_id: u64, location: Point2D) {
        // If there is already a definition
        if !self.contains_id(figure_id) {
            panic!("Definition not found!")
        }

        self.add_use_to(name, &Renderer::get_id_of_figure(figure_id)[..], location)
            .expect("Failed to add figure to container!")
    }

    /// Deletes a named item from the DOM and from internal entries.
    /// But will not delete definitions
    ///
    /// # Arguments
    /// * `name` - Name of item to delete
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));
    ///
    /// // --snip
    ///
    /// // Delete the named figure
    /// renderer.delete_named("named_circle");
    ///
    /// // Renders the circle with "named_circle" name again.
    /// // This would normally panic, since a name is redeclared,
    /// // but since the named figure is deleted, it will not. :)
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));
    /// ```
    pub fn delete_named(&mut self, name: &str) {
        let container = self.get_named_container(name);

        if !container.is_err() {
            let container = container.unwrap();

            let parent = container
                .parent_element()
                .ok_or(NoParent)
                .expect("No parent was found!");

            parent
                .remove_child(&container)
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Will be set to false
    /// let does_named_circle_exist = renderer.does_name_exist("named_circle");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));
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
    /// renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Adds the named container 'named_container' to the svg root
    /// renderer.create_named_container("named_container", "root");
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.append_to_container("named_container", circle, (10.0, 10.0));
    /// ```
    pub fn create_named_container(&mut self, name: &str, parent: &str) {
        let parent = self
            .get_named_container(parent)
            .expect("Failed to fetch named container!");

        let id_string = self
            .create_id_string(name)
            .expect("Unable to create id string");

        let container = crate::create_element_ns("http://www.w3.org/2000/svg", "g")
            .expect("Failed to create new named container!");
        container.set_id(&id_string[..]);

        parent
            .append_child(&container)
            .map_err(|_| Dom(UnappendableElement))
            .expect("Failed to append named container to parent!");
    }

    /// Moves a named figure to a given location
    ///
    /// # Arguments
    /// * `name` - Name of the named figure to move
    /// * `loc` - Location to move the figure to
    ///
    /// # Examples
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", circle, (10.0, 10.0));
    ///
    /// // --snip
    ///
    /// // Moves the named figure to a new location
    /// renderer.move_named("named_circle", (5.0, 5.0));
    /// ```
    pub fn move_named(&mut self, name: &str, loc: Point2D) {
        if !self.does_name_exist(name) {
            panic!("Failed to move named figure: Name doesn't exist!");
        }

        if self.is_container(name) {
            panic!("Failed to move named figure: Name is used for a container!")
        }

        let element = super::get_document()
            .expect("Document failed")
            .get_element_by_id(&Renderer::get_id_of_named(self.name_defs.get(name).unwrap())[..])
            .unwrap();

        element
            .set_attribute("x", &format!("{:.2}", loc.0)[..])
            .unwrap();
        element
            .set_attribute("y", &format!("{:.2}", loc.1)[..])
            .unwrap();
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer (must be mutable)
    /// let mut renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Generate circle
    /// let circle = SVGDefault::circle(10);
    ///
    /// // Render circle (since it's the first time of rendering this shape,
    /// // the renderer will add the shape's definition)
    /// renderer.render_named("named_circle", circle, (10.0, 10.0));
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
    /// ```rust,no_run
    /// use wasm_svg_graphics::prelude::*;
    ///
    /// // Declare renderer
    /// let renderer = SVGRenderer::new("svg_parent_id")
    ///     .expect("Failed to create renderer!");
    ///
    /// // Adjust the viewbox
    /// renderer.adjust_viewbox(0, 0, 50, 50);
    /// ```
    pub fn adjust_viewbox(&self, x: i32, y: i32, width: i32, height: i32) {
        Renderer::set_view_box(
            &self
                .get_svg_root()
                .expect("Failed to retrieve SVG container!"),
            x,
            y,
            width,
            height,
        )
        .expect("Failed to set viewBox!");
    }
}
