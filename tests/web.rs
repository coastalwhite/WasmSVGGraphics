//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("Cant find window");

    window.document().expect("Cant find Document")
}

fn add_svg_parent() {
    let prev_parent = get_document().get_element_by_id("svg_parent_id");

    match prev_parent {
        Some(el) => {
            el.set_inner_html("");
        }
        None => {
            let parent = get_document()
                .create_element("div")
                .expect("Couldn't create parent");

            parent.set_id("svg_parent_id");
            get_document()
                .body()
                .expect("Document doenst contain body")
                .append_child(&parent)
                .expect("Failed to add parent");
        }
    }
}

#[wasm_bindgen_test]
fn lib_simple_test() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable) into a parent container
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render(circle, (20.0, 20.0));
}

#[wasm_bindgen_test]
fn lib_simple_smiley() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable) into a parent container
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    let smiley = SVGElem::new(Tag::Group)
        .append(SVGDefault::circle(20))
        .append(SVGDefault::set_circle_loc(SVGDefault::circle(3), -7, -7))
        .append(SVGDefault::set_circle_loc(SVGDefault::circle(3), 7, -7))
        .append(SVGDefault::curve(-7, 5, 7, 5, -4, 10, 4, 10));

    renderer.render(smiley, (25.0, 25.0));
}

#[wasm_bindgen_test]
fn lib_simple_colored_smiley() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable) into a parent container
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    let colored_smiley = SVGElem::new(Tag::Group)
        .append(SVGDefault::circle(20).set(Attr::StrokeColor, RGB::new(255, 255, 0).into()))
        .append(
            SVGDefault::set_circle_loc(SVGDefault::circle(3), -7, -7)
                .set(Attr::FillColor, RGB::new(0, 0, 0).into()),
        )
        .append(
            SVGDefault::set_circle_loc(SVGDefault::circle(3), 7, -7)
                .set(Attr::FillColor, RGB::new(0, 0, 0).into()),
        )
        .append(
            SVGDefault::curve(-7, 5, 7, 5, -4, 10, 4, 10)
                .set(Attr::StrokeColor, RGB::new(255, 0, 0).into())
                .set(Attr::FillColor, RGBT::Transparent.into()),
        );

    renderer.render(colored_smiley, (25.0, 25.0));
}

#[wasm_bindgen_test]
fn renderer_render() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render(circle, (20.0, 20.0));
}
#[wasm_bindgen_test]
fn renderer_render_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", circle, (10.0, 10.0));

    // --snip

    // Updates the named figure's location to (20,20)
    renderer.move_named("named_circle", (20.0, 20.0));
}

#[wasm_bindgen_test]
fn renderer_render_id() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    let circle_id = renderer.define_render(circle);

    // Render circle
    renderer.render_id(circle_id, (20.0, 20.0));
}

#[wasm_bindgen_test]
fn renderer_named_id() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    let circle_id = renderer.define_render(circle);

    // Render circle
    renderer.render_named_id("named_circle", circle_id, (20.0, 20.0));

    // --snip

    // Updates the Circle's location
    renderer.move_named("named_circle", (25.0, 25.0));
}

#[wasm_bindgen_test]
fn renderer_define_render() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Define the render
    let circle_id = renderer.define_render(circle);

    // Render circle
    renderer.render_id(circle_id, (20.0, 20.0));
}

#[wasm_bindgen_test]
fn renderer_clear() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));

    // --snip

    // Reinit the Renderer object
    renderer.clear();

    // Renders the circle with "named_circle" name again.
    // This would normally panic, since a name is redeclared,
    // but since the renderer is cleared, it will not. :)
    renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));
}
#[wasm_bindgen_test]
fn renderer_clear_named_container() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));

    // Now the container contains the circle figure

    // --snip

    // Clear the named container
    renderer.clear_named_container("named_container");

    // Now the container is empty again

    // Render circle in the named_container.
    // Since definitions were not cleared, it will use a previous definition.
    // This saves some processing time in this case.
    renderer.append_to_container("named_container", SVGDefault::circle(10), (20.0, 20.0));

    // Now the container contains the circle at a different position
}

#[wasm_bindgen_test]
fn renderer_update_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));

    // Now the container contains the circle figure

    // --snip

    // Update the contents of the named container
    renderer.update_named("named_container", SVGDefault::circle(10), (20.0, 20.0));

    // Now the container contains the circle at a different position
}

#[wasm_bindgen_test]
fn renderer_update_named_with_id() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    let circle_id = renderer.define_render(SVGDefault::circle(10));

    // Render circle
    renderer.append_to_container("named_container", SVGDefault::circle(10), (10.0, 10.0));

    // Now the container contains the circle figure

    // --snip

    // Update the contents of the named container
    renderer.update_named_with_id("named_container", circle_id, (20.0, 20.0));

    // Now the container contains the circle at a different position
}

#[wasm_bindgen_test]
fn renderer_hide_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", circle, (10.0, 10.0));

    // --snip

    // Hides the named figure
    renderer.hide_named("named_circle");
}
#[wasm_bindgen_test]
fn renderer_show_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", circle, (10.0, 10.0));

    // Hides the named figure
    renderer.hide_named("named_circle");

    // --snip

    // Show the named figure again
    renderer.show_named("named_circle");
}

#[wasm_bindgen_test]
fn renderer_append_to_container() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.append_to_container("named_container", circle, (10.0, 10.0));

    // Now the container contains the circle figure
}

#[wasm_bindgen_test]
fn renderer_append_to_container_with_id() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    let circle_id = renderer.define_render(SVGDefault::circle(10));

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.append_to_container_with_id("named_container", circle_id, (10.0, 10.0));

    // Now the container contains the circle figure
}

#[wasm_bindgen_test]
fn renderer_delete_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));

    // --snip

    // Delete the named figure
    renderer.delete_named("named_circle");

    // Renders the circle with "named_circle" name again.
    // This would normally panic, since a name is redeclared,
    // but since the named figure is deleted, it will not. :)
    renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));
}

#[wasm_bindgen_test]
fn renderer_does_named_exist() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Will be set to false
    let does_named_circle_exist = renderer.does_name_exist("named_circle");
    assert!(!does_named_circle_exist);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", SVGDefault::circle(10), (10.0, 10.0));

    // Will be set to true
    let does_named_circle_exist = renderer.does_name_exist("named_circle");
    assert!(does_named_circle_exist);

    // Delete the named figure
    renderer.delete_named("named_circle");

    // Will be set to false
    let does_named_circle_exist = renderer.does_name_exist("named_circle");
    assert!(!does_named_circle_exist);

    // Renders the circle with "named_circle" name again.
    // This would normally panic, since a name is redeclared,
    // but since the named figure is deleted, it will not. :)
    renderer.render_named("named_circle", SVGDefault::circle(10), (20.0, 20.0));

    // Will be set to true
    let does_named_circle_exist = renderer.does_name_exist("named_circle");
    assert!(does_named_circle_exist);
}

#[wasm_bindgen_test]
fn renderer_create_named_container() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Adds the named container 'named_container' to the svg root
    renderer.create_named_container("named_container", "root");

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.append_to_container("named_container", circle, (10.0, 10.0));
}

#[wasm_bindgen_test]
fn renderer_move_named() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", circle, (10.0, 10.0));

    // --snip

    // Moves the named figure to a new location
    renderer.move_named("named_circle", (5.0, 5.0));
}

#[wasm_bindgen_test]
fn renderer_is_container() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer (must be mutable)
    let mut renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Generate circle
    let circle = SVGDefault::circle(10);

    // Render circle (since it's the first time of rendering this shape,
    // the renderer will add the shape's definition)
    renderer.render_named("named_circle", circle, (10.0, 10.0));

    // Create a named container
    renderer.create_named_container("named_container", "root");

    println!("{}", renderer.is_container("named_circle")); // false
    println!("{}", renderer.is_container("named_container")); // true
    println!("{}", renderer.is_container("not_in_use_name")); // false
}

#[wasm_bindgen_test]
fn renderer_adjust_viewbox() {
    add_svg_parent();

    use wasm_svg_graphics::prelude::*;

    // Declare renderer
    let renderer = SVGRenderer::new("svg_parent_id").expect("Failed to create renderer!");

    // Adjust the viewbox
    renderer.adjust_viewbox(0, 0, 50, 50);
}
