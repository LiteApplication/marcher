#[macro_use]
extern crate glium;

mod graphical_types;
mod ui;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let mut app = ui::Application::build(&event_loop).expect("could not initialise app");
    event_loop
        .run_app(&mut app)
        .expect("Error while running the app")
}
