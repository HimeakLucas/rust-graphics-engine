mod shader;
use shader::Shader;
mod material;
use material::Material;
mod camera;
use camera::{Camera, CameraMovement};
use cgmath::Point3;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::{Api, ContextBuilder, GlRequest};

use glutin::window::{WindowBuilder, CursorGrabMode};
use std::ptr;

use cgmath::{Matrix3, Matrix4, Vector3, Deg, SquareMatrix, Matrix, perspective};
use std::time::Instant;
use glutin::event::VirtualKeyCode;

use glutin::event::{DeviceEvent, ElementState}; // Adicione DeviceEvent e ElementState

fn main() {
    
    let start_time = Instant::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("LWA-Graphics-Engine");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3,3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);


    gl_context.window()
        .set_cursor_grab(CursorGrabMode::Locked)
        .or_else(|_| gl_context.window().set_cursor_grab(CursorGrabMode::Confined)) // Fallback seguro
        .expect("Failed to grab cursor");

    gl_context.window().set_cursor_visible(false);



    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    // obs: r#" "# é uma raw string literal. Não é necessário \n ou \". A string aparece exatamente
    // como está entre aspas

    let lighting_shader = Shader::new("resources/shaders/basic_lighting.vs", "resources/shaders/basic_lighting.fs")
        .expect("Failed to create lighting shader");

    let light_cube_shader = Shader::new("resources/shaders/light_cube.vs", "resources/shaders/light_cube.fs")
        .expect("Failed to create light cube shader");

    let mut camera = Camera::new(

        Point3::new(0.0, 0.0, 3.0),
        -90.0,
        0.0
    );

    let mut w_pressed = false;
    let mut s_pressed = false;
    let mut a_pressed = false;
    let mut d_pressed = false;
    
    let mut first_mouse = true;
    let mut last_x = 400.0;
    let mut last_y = 300.0;


// Criando um material de "Esmeralda" (exemplo)
    let emerald = Material::new(
        Vector3::new(0.0215, 0.1745, 0.0215),
        Vector3::new(0.07568, 0.61424, 0.07568),
        Vector3::new(0.633, 0.727811, 0.633),
        0.6 * 128.0
    );

    let gold = Material::new(
            Vector3::new(0.24725, 0.1995, 0.0745),
            Vector3::new(0.75164, 0.60648, 0.22648),
            Vector3::new(0.62828, 0.55580, 0.36606),
            51.0
    );


//Vertices e normais
let vertices: [f32; 216] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];

     //Inicia duas variáveis mutáveis e elas vão ser reescritas por funções do opengl, então não importa o valor inicial.
    let mut vbo: u32 = 0;
    let mut cube_vao: u32 = 0;
    let mut light_cube_vao: u32 = 0;

    unsafe {

        gl::GenBuffers(1, &mut vbo); //Cria 1 unidade de buffer e atribui um id à vbo para o buffer gerado

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //A partir deste ponto, qualquer chamada de buffer
        //vai ser usada para configurar o atual bound buffer.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        //Cubo principal
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::BindVertexArray(cube_vao);

        let stride =(6 * std::mem::size_of::<f32>()) as gl::types::GLint;//strinde
        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            0, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            stride,
            ptr::null(), //offset (posição os os dados começam no buffer)
        );
        gl::EnableVertexAttribArray(0);

        let offset = (3 * std::mem::size_of::<f32>()) as *const _ ;
        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            1, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            stride,
            offset, //offset (posição os os dados começam no buffer)
        );
        gl::EnableVertexAttribArray(1);


        //Cubo de luz
        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)

        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            0, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            stride,
            ptr::null(), //offset (posição os os dados começam no buffer)
        );
        gl::EnableVertexAttribArray(0);

    }

    let mut last_frame_time = 0.0;

    event_loop.run(move |event, _ , control_flow| { //??????

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {

                unsafe {
                    gl::DeleteVertexArrays(1, &cube_vao);
                    gl::DeleteVertexArrays(1, &light_cube_vao);
                    gl::DeleteBuffers(1, &vbo);
                }
                return;
            },
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(physical_size)  => {
                    gl_context.resize(physical_size);
                    unsafe {
                        gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                    }
                },

                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {input, ..} => {

                    if let Some(keycode) = input.virtual_keycode {

                        let is_pressed = input.state == glutin::event::ElementState::Pressed;

                        match keycode {
                            VirtualKeyCode::W => w_pressed = is_pressed,
                            VirtualKeyCode::S => s_pressed = is_pressed,
                            VirtualKeyCode::A => a_pressed = is_pressed,
                            VirtualKeyCode::D => d_pressed = is_pressed,
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            _ => (),
                        }
                    }
                }
                _ => (),
                }

            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                camera.process_mouse(delta.0 as f32, -delta.1 as f32); 
            },
        
            Event::RedrawRequested(_) => {


                let current_time = start_time.elapsed().as_secs_f32();
                let delta_time = current_time - last_frame_time;
                last_frame_time = current_time;

                if w_pressed {camera.process_keyboard(CameraMovement::Forward, delta_time);}
                if s_pressed {camera.process_keyboard(CameraMovement::Backward, delta_time);}
                if a_pressed {camera.process_keyboard(CameraMovement::Left, delta_time);}
                if d_pressed {camera.process_keyboard(CameraMovement::Right, delta_time);}

                let time_value = start_time.elapsed().as_secs_f32();

                unsafe {
                    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    let light_x = 3.5 * (time_value * 1.0).sin();
                    let light_y = 3.5 * (time_value * 1.0).cos();
                    
                    let light_pos = Vector3::new(light_x, 1.0, light_y);
                    
                    let light_color = Vector3::new(1.0, 1.0, 1.0);
                    let diffuse_color = light_color * 0.5;
                    let ambient_color = diffuse_color * 0.3;
                    

                    //desenha o cubo principal
                    lighting_shader.use_program();
                    lighting_shader.set_vec3("light.position", &light_pos);
                    lighting_shader.set_vec3("light.diffuse", &diffuse_color);
                    lighting_shader.set_vec3("light.ambient", &ambient_color);
                    lighting_shader.set_vec3("light.specular", &Vector3::new(1.0, 1.0, 1.0));

                    let cam_pos = Vector3::new(camera.position.x, camera.position.y, camera.position.z); 
                    lighting_shader.set_vec3("viewPos", &cam_pos);

                    
                    let view = camera.get_view_matrix();
                    let projection = perspective(Deg(45.0), 800.0 / 600.0, 0.1, 100.0);

                    lighting_shader.set_mat4("view", &view);
                    lighting_shader.set_mat4("projection", &projection);


                    emerald.apply(&lighting_shader, "material");                  
                    let mut model = Matrix4::from_translation(Vector3::new(1.0, 0.0, 0.0));
                    model = model * Matrix4::from_angle_y(Deg(-time_value * 15.0));
                    model = model * Matrix4::from_angle_x(Deg(-time_value * 13.0));

                    lighting_shader.set_mat4("model", &model);

                    let normal_matrix = Matrix3::from_cols(
                        model.x.truncate(),
                        model.y.truncate(),
                        model.z.truncate()
                    ).invert().unwrap().transpose();

                    lighting_shader.set_mat3("normalMatrix", &normal_matrix);

                    gl::BindVertexArray(cube_vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);

                    gold.apply(&lighting_shader, "material");
                    let mut model = Matrix4::from_translation(Vector3::new(-1.0, 0.0, 0.0));
                    model = model * Matrix4::from_angle_y(Deg(time_value * 10.0));
                    model = model * Matrix4::from_angle_x(Deg(time_value * 16.0));

                    lighting_shader.set_mat4("model", &model);

                    let normal_matrix = Matrix3::from_cols(
                        model.x.truncate(),
                        model.y.truncate(),
                        model.z.truncate()
                    ).invert().unwrap().transpose();

                    lighting_shader.set_mat3("normalMatrix", &normal_matrix);

                    gl::DrawArrays(gl::TRIANGLES, 0, 36);


                    //desenha o cubo lampada
                    light_cube_shader.use_program();
                    light_cube_shader.set_mat4("projection", &projection);
                    light_cube_shader.set_mat4("view", &view);


                    //aplica uma transformação que primeiro move a lampada do centro e demois
                    //reescala
                    let mut model = Matrix4::from_translation(light_pos);
                    model = model * Matrix4::from_scale(0.2);

                    light_cube_shader.set_mat4("model", &model);

                    gl::BindVertexArray(light_cube_vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 36)

                }
                gl_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                gl_context.window().request_redraw();
            }
            _ => (),
        }
    });
}
