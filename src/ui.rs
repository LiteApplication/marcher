mod camera;

use super::fragment_shader_builder::FragmentBuilder;
use std::{collections::HashMap, fs, time::Instant};

use glium::{
    Surface, VertexBuffer,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event::{DeviceEvent, ElementState, KeyEvent, WindowEvent},
        event_loop::EventLoop,
        keyboard::{KeyCode, PhysicalKey::Code},
        window::{self, Window},
    },
};

use crate::{graphical_types::Vertex, ui::camera::MovementDirection};

use anyhow::{Context, Result};

pub(crate) struct Application {
    window: Window,
    display: glium::backend::glutin::Display<WindowSurface>,

    shader: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    pressed_keys: HashMap<KeyCode, bool>,
    camera: camera::Camera,

    last_cursor_position: PhysicalPosition<f32>,
    start_time: Instant,
}

impl Application {
    pub fn build(
        event_loop: &EventLoop<()>,
        fragment_builder: impl FragmentBuilder,
    ) -> Result<Application> {
        //let (window, display) =
        //    glium::backend::glutin::SimpleWindowBuilder::new().build(event_loop);
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(event_loop);
        window
            .set_cursor_grab(window::CursorGrabMode::Locked)
            .or_else(|_| window.set_cursor_grab(window::CursorGrabMode::Confined))
            .expect("unable to lock cursor in place");
        window.set_cursor_visible(false);

        let vertex_buffer = Self::create_fragment_display(&display)
            .context("could not create the fragment display")?;
        let shader = Self::init_shader(&display, fragment_builder)
            .context("could not initialize the shader")?;
        let camera = camera::Camera::new();

        Ok(Application {
            window,
            display,
            vertex_buffer,
            shader,
            pressed_keys: HashMap::new(),
            camera,
            last_cursor_position: PhysicalPosition { x: 0.0, y: 0.0 },
            start_time: Instant::now(),
        })
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        let code = match key_event.physical_key {
            Code(code) => code,
            _ => return,
        };
        self.pressed_keys
            .insert(code, key_event.state == ElementState::Pressed);
    }

    fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.get(&key).unwrap_or(&false).clone()
    }

    fn create_fragment_display(
        display: &glium::backend::glutin::Display<WindowSurface>,
    ) -> Result<VertexBuffer<Vertex>> {
        let vertex1 = Vertex {
            position: [-1.0, -3.0],
            //color: [3.0, 0.0, 0.0],
        };
        let vertex2 = Vertex {
            position: [-1.0, 1.0],
            //color: [0.0, 1.0, 0.0],
        };
        let vertex3 = Vertex {
            position: [3.0, 1.0],
            //color: [0.0, 0.0, 3.0],
        };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape)
            .context("could not create the vertex buffer")?;
        Ok(vertex_buffer)
    }

    fn init_shader(
        display: &glium::backend::glutin::Display<WindowSurface>,
        fragment_builder: impl FragmentBuilder,
    ) -> anyhow::Result<glium::Program> {
        // Start by loading the shaders' code
        let vertex_shader_src = fs::read_to_string("shaders/simple/vertex.glsl")
            .context("could not read the vertex shader file")?;

        let program = glium::Program::from_source(
            display,
            vertex_shader_src.as_str(),
            &fragment_builder.export(),
            None,
        )
        .context("could not compile shaders")?;

        Ok(program)
    }

    fn redraw(&mut self) {
        // Let's start with input handling

        if self.is_pressed(glium::winit::keyboard::KeyCode::KeyW) {
            self.camera
                .move_in_direction(MovementDirection::Front, 1.0 / 60.0); // TODO: Real timing
        } else if self.is_pressed(glium::winit::keyboard::KeyCode::KeyS) {
            self.camera
                .move_in_direction(MovementDirection::Back, 1.0 / 60.0);
        }
        if self.is_pressed(glium::winit::keyboard::KeyCode::KeyA) {
            self.camera
                .move_in_direction(MovementDirection::Left, 1.0 / 60.0);
        } else if self.is_pressed(glium::winit::keyboard::KeyCode::KeyD) {
            self.camera
                .move_in_direction(MovementDirection::Right, 1.0 / 60.0);
        }
        if self.is_pressed(glium::winit::keyboard::KeyCode::Space) {
            self.camera
                .move_in_direction(MovementDirection::Up, 1.0 / 60.0);
        } else if self.is_pressed(glium::winit::keyboard::KeyCode::ShiftLeft) {
            self.camera
                .move_in_direction(MovementDirection::Down, 1.0 / 60.0);
        }

        // Then we draw the frame

        let mut frame = self.display.draw();
        let (width, height) = frame.get_dimensions();
        let screen_size: [f32; 2] = [width as f32, height as f32]; // This will be sent over to the GPU

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                &self.vertex_buffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.shader,
                &uniform! {
                    screen_size: screen_size,
                    camera_position: self.camera.get_position(),
                    camera_rotation_mat: self.camera.get_rotation_matrix(),
                    seed_time: self.start_time.elapsed().as_millis() as u32 // i don't care if it wraps around
                },
                &Default::default(),
            )
            .context("unable to draw the frame")
            .unwrap();

        frame.finish().unwrap();
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        ()
    }

    fn about_to_wait(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        self.window.request_redraw(); // This is needed to have frames continuously
    }

    fn device_event(
        &mut self,
        _event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _device_id: glium::winit::event::DeviceId,
        event: glium::winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let (d_x, d_y) = delta;
                self.camera.mouse_moved(d_x as f32, d_y as f32);
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(window_size) => {
                self.display.resize(window_size.into());
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent { repeat: false, .. } => self.handle_input(event),
                _ => (), // Ignore OS-level repetition for held keys
            },
            WindowEvent::CursorMoved { position, .. } => {
                let new_position: PhysicalPosition<f32> = PhysicalPosition {
                    x: position.x as f32,
                    y: position.y as f32,
                };
                self.camera.mouse_moved(
                    new_position.x - self.last_cursor_position.x,
                    new_position.y - self.last_cursor_position.y,
                );
                self.last_cursor_position = new_position;
            }

            _ => (),
        }
    }
}
