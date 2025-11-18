mod shader;
use shader::Shader;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use std::ptr;

use cgmath::{Matrix4, Vector3, Deg, SquareMatrix, Matrix, perspective};
use std::time::Instant;



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

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    // obs: r#" "# é uma raw string literal. Não é necessário \n ou \". A string aparece exatamente
    // como está entre aspas

    let shader = Shader::new("src/solid_color.vs", "src/solid_color.fs")
        .expect("Failed to create shaders");


let vertices: [f32; 108] = [ // 36 vértices * 3 floats (posição)
        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

         0.5,  0.5,  0.5,
         0.5,  0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5,  0.5,  0.5,

        -0.5, -0.5, -0.5,
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
         0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5,  0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ];


     //Inicia duas variáveis mutáveis e elas vão ser reescritas por funções do opengl, então não importa o valor inicial.
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;

    unsafe {
        gl::GenVertexArrays(1, & mut vao);
        gl::GenBuffers(1, &mut vbo); //Cria 1 unidade de buffer e atribui um id à vbo para o buffer
        //gerado

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //A partir deste ponto, qualquer chamada de buffer
        //vai ser usada para configurar o atual bound buffer.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        let stride =(3 * std::mem::size_of::<f32>()) as gl::types::GLint;//strinde

        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            0, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            stride,
            ptr::null(), //offset (posição os os dados começam no buffer)
        );
        gl::EnableVertexAttribArray(0);

        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)

    }



    event_loop.run(move |event, _ , control_flow| { //??????

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {

                unsafe {
                    gl::DeleteVertexArrays(1, &vao);
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
                _ => (),

            },
            Event::RedrawRequested(_) => {

                let time_value = start_time.elapsed().as_secs_f32();

                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                    shader.use_program();
                    let mut model = Matrix4::from_angle_x(Deg(time_value * 50.0));
                    model = model * Matrix4::from_angle_y(Deg(time_value * 30.0));
                    let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0));
                    let projection = perspective(Deg(45.0), 800.0 / 600.0, 0.1, 100.0);

                    shader.set_mat4("model", &model);
                    shader.set_mat4("view", &view);
                    shader.set_mat4("projection", &projection);

                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
