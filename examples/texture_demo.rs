use rust_graphics_engine::shader::Shader;
use rust_graphics_engine::material::Material;
use rust_graphics_engine::camera::{Camera, CameraMovement};
use rust_graphics_engine::texture::Texture; // Importante
use cgmath::Point3;

use glutin::event::{Event, WindowEvent, DeviceEvent, ElementState}; // ElementState adicionado
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::{Api, ContextBuilder, GlRequest};
use glutin::window::{WindowBuilder, CursorGrabMode};
use glutin::event::VirtualKeyCode;

use std::ptr;
use cgmath::{Matrix3, Matrix4, Vector3, Deg, SquareMatrix, Matrix, perspective};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("LWA-Graphics-Engine");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    // Configuração do Mouse inicial
    let window = gl_context.window();
    window.set_cursor_grab(CursorGrabMode::Locked)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined))
        .expect("Failed to grab cursor");
    window.set_cursor_visible(false);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // --- SHADERS ---
    // Certifique-se que texture.vs e texture.fs estão com o código Híbrido que criamos
    let lighting_shader = Shader::new("resources/shaders/texture.vs", "resources/shaders/texture.fs")
        .expect("Failed to create lighting shader");

    let light_cube_shader = Shader::new("resources/shaders/light_cube.vs", "resources/shaders/light_cube.fs")
        .expect("Failed to create light cube shader");

    // --- TEXTURAS ---
    // Certifique-se de ter uma imagem aqui
    let texture_1 = Texture::new("resources/textures/wood2.jpg"); 
    let texture_2 = Texture::new("resources/textures/wood5.jpg");

    // --- CÂMERA ---
    let mut camera = Camera::new(
        Point3::new(0.0, 0.0, 3.0),
        -90.0,
        0.0
    );

    // --- CONTROLES ---
    let mut w_pressed = false;
    let mut s_pressed = false;
    let mut a_pressed = false;
    let mut d_pressed = false;
    let mut space_pressed = false;
    let mut shift_pressed = false;

    let mut last_frame_time = 0.0;
    let mut simulated_time = 0.0;
    let mut world_paused = false;

    // --- MATERIAIS ---
    
    // 1. Material COM Textura (As cores sólidas são ignoradas pelo shader quando tem textura)
    let textured_material = Material::new(
        Vector3::new(0.0, 0.0, 0.0), // Ambient (ignorado)
        Vector3::new(0.0, 0.0, 0.0), // Diffuse (ignorado)
        Vector3::new(0.5, 0.5, 0.5), // Specular
        32.0,                        // Shininess
        Some(0)                      // <--- USA TEXTURA NA UNIDADE 0
    );

    // 2. Material SÓLIDO (Ouro)
    let gold_material = Material::new(
        Vector3::new(0.24725, 0.1995, 0.0745),   // Ambient
        Vector3::new(0.75164, 0.60648, 0.22648), // Diffuse
        Vector3::new(0.62828, 0.55580, 0.36606), // Specular
        51.2,                                    // Shininess
        Some(1)                                     // <--- NÃO USA TEXTURA (Cor sólida)
    );

    // --- VERTICES (Com UVs) ---
    let vertices: [f32; 288] = [
        // Pos (3)            // Normal (3)      // Tex (2)
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
    ];

    let mut vbo: u32 = 0;
    let mut cube_vao: u32 = 0;
    let mut light_cube_vao: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        // --- VAO DO CUBO PRINCIPAL ---
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::BindVertexArray(cube_vao);

        // Stride agora é 8 (3 Pos + 3 Norm + 2 Tex)
        let stride = (8 * std::mem::size_of::<f32>()) as gl::types::GLint;

        // 1. Posição (Location 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // 2. Normal (Location 1) - Offset de 3 floats
        let offset_norm = (3 * std::mem::size_of::<f32>()) as *const _;
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, offset_norm);
        gl::EnableVertexAttribArray(1);

        // 3. Textura (Location 2) - Offset de 6 floats (3 pos + 3 norm) <--- IMPORTANTE
        let offset_tex = (6 * std::mem::size_of::<f32>()) as *const _;
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, offset_tex);
        gl::EnableVertexAttribArray(2);


        // --- VAO DA LÂMPADA ---
        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Lampada só precisa de posição (Location 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // --- LOOP PRINCIPAL ---
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {
                unsafe {
                    gl::DeleteVertexArrays(1, &cube_vao);
                    gl::DeleteVertexArrays(1, &light_cube_vao);
                    gl::DeleteBuffers(1, &vbo);
                }
            },

            // --- EVENTOS DE JANELA E TECLADO ---
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    gl_context.resize(physical_size);
                    unsafe {
                        gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                    }
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        let is_pressed = input.state == ElementState::Pressed;
                        match keycode {
                            VirtualKeyCode::W => w_pressed = is_pressed,
                            VirtualKeyCode::S => s_pressed = is_pressed,
                            VirtualKeyCode::A => a_pressed = is_pressed,
                            VirtualKeyCode::D => d_pressed = is_pressed,
                            VirtualKeyCode::Space => space_pressed = is_pressed,
                            VirtualKeyCode::LShift => shift_pressed = is_pressed,
                            
                            // Logica do Pause (P)
                            VirtualKeyCode::P => {
                                if is_pressed {
                                    world_paused = !world_paused;
                                    // Não soltamos o mouse para permitir voar congelado
                                }
                            },
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            _ => (),
                        }
                    }
                },
                _ => (),
            },

            // --- MOVIMENTO DO MOUSE (RAW) ---
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                camera.process_mouse(delta.0 as f32, -delta.1 as f32);
            },

            // --- RENDERIZAÇÃO ---
            Event::RedrawRequested(_) => {
                // 1. Tempo Real e Delta (Para a câmera)
                let current_time = start_time.elapsed().as_secs_f32();
                let delta_time = current_time - last_frame_time;
                last_frame_time = current_time;

                // 2. Tempo Simulado (Para a física do mundo)
                if !world_paused {
                    simulated_time += delta_time;
                }

                // 3. Processa Movimento da Câmera
                if w_pressed { camera.process_keyboard(CameraMovement::Forward, delta_time); }
                if s_pressed { camera.process_keyboard(CameraMovement::Backward, delta_time); }
                if a_pressed { camera.process_keyboard(CameraMovement::Left, delta_time); }
                if d_pressed { camera.process_keyboard(CameraMovement::Right, delta_time); }
                if space_pressed { camera.process_keyboard(CameraMovement::Up, delta_time); }
                if shift_pressed { camera.process_keyboard(CameraMovement::Down, delta_time); }


                // 4. Desenho
                unsafe {
                    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    // --- SETUP DA LUZ E CÂMERA ---
                    let light_x = 3.5 * (simulated_time * 1.0).sin();
                    let light_y = 3.5 * (simulated_time * 1.0).cos();
                    let light_pos = Vector3::new(light_x, 1.0, light_y);

                    lighting_shader.use_program();
                    lighting_shader.set_vec3("light.position", &light_pos);
                    lighting_shader.set_vec3("light.diffuse", &Vector3::new(0.5, 0.5, 0.5));
                    lighting_shader.set_vec3("light.ambient", &Vector3::new(0.2, 0.2, 0.2));
                    lighting_shader.set_vec3("light.specular", &Vector3::new(1.0, 1.0, 1.0));

                    let cam_pos = Vector3::new(camera.position.x, camera.position.y, camera.position.z);
                    lighting_shader.set_vec3("viewPos", &cam_pos);

                    let view = camera.get_view_matrix();
                    let projection = perspective(Deg(45.0), 800.0 / 600.0, 0.1, 100.0);
                    lighting_shader.set_mat4("view", &view);
                    lighting_shader.set_mat4("projection", &projection);
                    
                    gl::BindVertexArray(cube_vao);


                    // --- OBJETO 1: COM TEXTURA ---
                    // Ativa a unidade de textura 0 e liga nossa imagem
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture_1.id);
                    
                    textured_material.apply(&lighting_shader, "material");
                    
                    let mut model = Matrix4::from_translation(Vector3::new(1.0, 0.0, 0.0));
                    model = model * Matrix4::from_angle_y(Deg(-simulated_time * 15.0));
                    model = model * Matrix4::from_angle_x(Deg(-simulated_time * 13.0));
                    lighting_shader.set_mat4("model", &model);
                    
                    // Normal Matrix
                    let normal_matrix = Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate()).invert().unwrap().transpose();
                    lighting_shader.set_mat3("normalMatrix", &normal_matrix);
                    
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);


                    // --- OBJETO 2: OURO (SEM TEXTURA) ---
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, texture_2.id);
                    gold_material.apply(&lighting_shader, "material");
                    
                    let mut model = Matrix4::from_translation(Vector3::new(-1.0, 0.0, 0.0));
                    model = model * Matrix4::from_angle_y(Deg(simulated_time * 10.0));
                    model = model * Matrix4::from_angle_x(Deg(simulated_time * 16.0));
                    lighting_shader.set_mat4("model", &model);

                    let normal_matrix = Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate()).invert().unwrap().transpose();
                    lighting_shader.set_mat3("normalMatrix", &normal_matrix);

                    gl::DrawArrays(gl::TRIANGLES, 0, 36);


                    // --- LAMPADA ---
                    light_cube_shader.use_program();
                    light_cube_shader.set_mat4("projection", &projection);
                    light_cube_shader.set_mat4("view", &view);

                    let mut model = Matrix4::from_translation(light_pos);
                    model = model * Matrix4::from_scale(0.2);
                    light_cube_shader.set_mat4("model", &model);

                    gl::BindVertexArray(light_cube_vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
                gl_context.swap_buffers().unwrap();
            },

            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            },
            _ => (),
        }
    });
}
