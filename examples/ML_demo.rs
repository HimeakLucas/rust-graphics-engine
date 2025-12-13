use rust_graphics_engine::shader::Shader;
use rust_graphics_engine::model_loading::model::Model;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use cgmath::{Matrix4, Vector3, Deg, perspective};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("LWA-Graphics-Engine");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3,3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe { gl_context.make_current().unwrap() };
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    unsafe { gl::Enable(gl::DEPTH_TEST); }

    let shader = Shader::new("resources/shaders/solid_color.vs", "resources/shaders/solid_color.fs")
        .expect("Failed to create shaders");

    // Pass model Path here!
    let model = Model::load("resources/assets/miku.obj")
        .expect("Failed to load model");

    unsafe {gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)}
    


    event_loop.run(move |event, _, control_flow| {
        
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    gl_context.resize(size);
                    unsafe { gl::Viewport(0, 0, size.width as i32, size.height as i32) };
                }
                _ => (),
            },

            Event::RedrawRequested(_) => {
                let t = start_time.elapsed().as_secs_f32();

                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }

                shader.use_program();

                let model_matrix =
                    Matrix4::from_angle_y(Deg(t * 30.0)) *
                    Matrix4::from_angle_x(Deg(t * 50.0));

                let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0));
                let projection = perspective(Deg(45.0), 800.0 / 600.0, 0.1, 100.0);

                shader.set_mat4("model", &model_matrix);
                shader.set_mat4("view", &view);
                shader.set_mat4("projection", &projection);

                model.draw(shader.id);

                gl_context.swap_buffers().unwrap();
            }

            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            }

            _ => (),
        }
    });
}
