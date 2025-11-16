use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use std::ffi::CString; //representa uma owned string compatível com C. Facilita a interação com
//códiogo C de maneira segura (com funções estrangeiras (Foreign Function Interface = FFI))
use std::ptr;
use std::time::Instant;

const VERTEX_SHADER_SOURCE: &str = r#" 
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    void main() {
        gl_Position = vec4(aPos, 1.0);
        ourColor = aColor;
    }
    "#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 ourColor;

    void main() {
        FragColor = vec4(ourColor, 1.0);
    }
    "#;


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

    // obs: r#" "# é uma raw string literal. Não é necessário \n ou \". A string aparece exatamente
    // como está entre aspas

    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap(); //Transforma a string
        //Rust em uma string compatível com C. (Adiciona, por exemplo o \0 no final da string)
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null()); //Segundo argumento significa quantas
        //strings estamos passando como source code.
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); //Subtrai 1 para o terminador nulo \0
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERRO::SHADER::{}::COMPILAÇÃO_FALHOU\n{}", "VERTEX", String::from_utf8_lossy(&info_log));
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); //Subtrai 1 para o terminador nulo \0
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERRO::SHADER::{}::COMPILAÇÃO_FALHOU\n{}", "FRAGMENT", String::from_utf8_lossy(&info_log));
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); //Subtrai 1 para o terminador nulo \0
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
            println!("ERRO::PROGRAM::{}::LINK_FALHOU\n{}", "PROGRAM" ,String::from_utf8_lossy(&info_log));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        shader_program
    };
    


    let vertices: [f32; 18] = [
         0.5, -0.5,  0.0,     1.0, 0.0, 0.0,
        -0.5, -0.5,  0.0,     0.0, 1.0, 0.0,
         0.0,  0.5,  0.0,     0.0, 0.0, 1.0,
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

        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            1, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            stride,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)

    }



    event_loop.run(move |event, _ , control_flow| { //??????

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {
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


                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);


                    gl::UseProgram(shader_program);


                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3)
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
