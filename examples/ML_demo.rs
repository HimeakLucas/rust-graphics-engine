use rust_graphics_engine::shader::Shader;
use rust_graphics_engine::model_loading::model::Model;
use rust_graphics_engine::camera::{Camera, CameraMovement};
use rust_graphics_engine::material::Material;

use glutin::event::{Event, WindowEvent, DeviceEvent, VirtualKeyCode, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{WindowBuilder, CursorGrabMode};
use glutin::{Api, ContextBuilder, GlRequest};

use cgmath::{
    Matrix4, Matrix3,
    Vector3, Point3,
    Deg, SquareMatrix, perspective, Matrix,
};

use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    // ---------------- WINDOW + CONTEXT ----------------
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("LWA-Graphics-Engine");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe { gl_context.make_current().unwrap() };
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // ---------------- SHADERS ----------------
    let lighting_shader = Shader::new(
        "resources/shaders/basic_lighting.vs",
        "resources/shaders/basic_lighting.fs",
    ).expect("Failed to create lighting shader");

    let _light_cube_shader = Shader::new(
        "resources/shaders/light_cube.vs",
        "resources/shaders/light_cube.fs",
    ).expect("Failed to create light cube shader");

    // ---------------- MODEL ----------------
    let model = Model::load("resources/assets/miku.obj")
        .expect("Failed to load model");

    // ---------------- CAMERA ----------------
    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 4.0),
        -90.0,
        0.0,
    );

    let mut w_pressed = false;
    let mut s_pressed = false;
    let mut a_pressed = false;
    let mut d_pressed = false;

    let mut last_frame_time = 0.0;

    // ---------------- EVENT LOOP ----------------
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,

            // ---------------- WINDOW EVENTS ----------------
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(size) => {
                    gl_context.resize(size);
                    unsafe {
                        gl::Viewport(0, 0, size.width as i32, size.height as i32);
                    }
                }

                WindowEvent::Focused(true) => {
                    let window = gl_context.window();
                    if window.set_cursor_grab(CursorGrabMode::Locked).is_err() {
                        let _ = window.set_cursor_grab(CursorGrabMode::Confined);
                    }
                    window.set_cursor_visible(false);
                }

                WindowEvent::Focused(false) => {
                    let window = gl_context.window();
                    let _ = window.set_cursor_grab(CursorGrabMode::None);
                    window.set_cursor_visible(true);
                }

                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let pressed = input.state == ElementState::Pressed;

                        match key {
                            VirtualKeyCode::W => w_pressed = pressed,
                            VirtualKeyCode::S => s_pressed = pressed,
                            VirtualKeyCode::A => a_pressed = pressed,
                            VirtualKeyCode::D => d_pressed = pressed,
                            VirtualKeyCode::Escape if pressed => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => (),
                        }
                    }
                }

                _ => (),
            },

            // ---------------- MOUSE INPUT ----------------
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                camera.process_mouse(delta.0 as f32, -delta.1 as f32);
            }

            // ---------------- RENDER ----------------
            Event::RedrawRequested(_) => {
                let current_time = start_time.elapsed().as_secs_f32();
                let delta_time = current_time - last_frame_time;
                last_frame_time = current_time;

                if w_pressed {
                    camera.process_keyboard(CameraMovement::Forward, delta_time);
                }
                if s_pressed {
                    camera.process_keyboard(CameraMovement::Backward, delta_time);
                }
                if a_pressed {
                    camera.process_keyboard(CameraMovement::Left, delta_time);
                }
                if d_pressed {
                    camera.process_keyboard(CameraMovement::Right, delta_time);
                }

                unsafe {
                    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }

                // ---------------- MATRICES ----------------
                let view = camera.get_view_matrix();
                let projection = perspective(
                    Deg(45.0),
                    800.0 / 600.0,
                    0.1,
                    100.0,
                );

                let model_matrix =
    Matrix4::from_angle_x(Deg(90.0));

                let normal_matrix = Matrix3::from_cols(
                    model_matrix.x.truncate(),
                    model_matrix.y.truncate(),
                    model_matrix.z.truncate(),
                )
                .invert()
                .unwrap()
                .transpose();

                // ---------------- LIGHT ----------------
                let light_pos = Vector3::new(1.0, 1.0, 1.0);
                let light_color = Vector3::new(1.0, 1.0, 1.0);

                let diffuse_color = light_color * 0.8;
                let ambient_color = diffuse_color * 0.2;

                let camera_pos = Vector3::new(
                    camera.position.x,
                    camera.position.y,
                    camera.position.z,
                );

                // ---------------- SHADER UNIFORMS ----------------
                lighting_shader.use_program();

                lighting_shader.set_mat4("model", &model_matrix);
                lighting_shader.set_mat4("view", &view);
                lighting_shader.set_mat4("projection", &projection);
                lighting_shader.set_mat3("normalMatrix", &normal_matrix);

                lighting_shader.set_vec3("viewPos", &camera_pos);

                lighting_shader.set_vec3("light.position", &light_pos);
                lighting_shader.set_vec3("light.ambient", &ambient_color);
                lighting_shader.set_vec3("light.diffuse", &diffuse_color);
                lighting_shader.set_vec3("light.specular", &Vector3::new(1.0, 1.0, 1.0));

                lighting_shader.set_vec3("material.ambient", &Vector3::new(0.2, 0.2, 0.2));
                lighting_shader.set_vec3("material.diffuse", &Vector3::new(0.525, 0.808, 0.796));
                lighting_shader.set_vec3("material.specular", &Vector3::new(1.0, 1.0, 1.0));
                lighting_shader.set_float("material.shininess", 32.0);

                // ---------------- DRAW ----------------
                model.draw(lighting_shader.id);

                gl_context.swap_buffers().unwrap();
            }

            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            }

            _ => (),
        }
    });
}
