use fragment_shader_builder::TestFragmentBuilder;

use crate::fragment_shader_builder::{
    FragmentBuilder,
    materials::{MaterialList, SolidColoredSurface},
    scene::{InfiniteYPlane, Scene, SceneObject, Sphere},
};

#[macro_use]
extern crate glium;

mod fragment_shader_builder;
mod graphical_types;
mod ui;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let mut materials = MaterialList::new();

    materials.add_material(
        "colored_thingy".to_string(),
        SolidColoredSurface::build(0.0, 1.0, 1.0).unwrap(),
    );
    materials.add_material(
        "colored_thingy2".to_string(),
        SolidColoredSurface::build(1.0, 0.5, 1.0).unwrap(),
    );

    materials.add_material(
        "floor".to_string(),
        SolidColoredSurface::build(0.7, 1.0, 0.8).unwrap(),
    );

    let mut scene = Scene::new();

    scene.add_object(
        Sphere::build(
            [0.0, 1.0, 2.0],
            1.0,
            materials
                .get_material(&"colored_thingy".to_string())
                .unwrap(),
        )
        .unwrap(),
    );
    scene.add_object(
        Sphere::build(
            [1.0, 0.0, 5.0],
            2.0,
            materials
                .get_material(&"colored_thingy2".to_string())
                .unwrap(),
        )
        .unwrap(),
    );

    scene.add_object(InfiniteYPlane::new(
        -0.5,
        materials.get_material(&"floor".to_string()).unwrap(),
    ));
    let fragment_builder = TestFragmentBuilder::new(materials, scene);

    fragment_builder.debug_print();

    let mut app =
        ui::Application::build(&event_loop, fragment_builder).expect("could not initialise app");
    event_loop
        .run_app(&mut app)
        .expect("Error while running the app");

    let _ = materials;
}
