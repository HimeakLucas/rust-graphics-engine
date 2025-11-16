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
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
    "#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec4 ourColor;

    void main() {
        FragColor = ourColor;
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
    
    let vertex_color_location = unsafe {
        let c_str = CString::new("ourColor").unwrap();
        gl::GetUniformLocation(shader_program, c_str.as_ptr())
    };


    let vertices: [f32; 12] = [
         0.5,  0.5, 0.0,
         0.5, -0.5, 0.0, 
        -0.5, -0.5, 0.0,
        -0.5,  0.5, 0.0 
    ];
    
    let indices: [u32; 6] = [
        0, 1, 3, //Primeiro triangulo
        1, 2, 3 // segundo triangulo
    ];

     //Inicia duas variáveis mutáveis e elas vão ser reescritas por funções do opengl, então não importa o valor inicial.
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    unsafe {
        gl::GenVertexArrays(1, & mut vao);
        gl::GenBuffers(1, &mut vbo); //Cria 1 unidade de buffer e atribui um id à vbo para o buffer
        //gerado
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //A partir deste ponto, qualquer chamada de buffer
        //vai ser usada para configurar o atual bound buffer.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
        

        gl::VertexAttribPointer( //Em relação ao current bounded buffer
            0, //layout (location = 0)
            3, // size (vec3)
            gl::FLOAT,
            gl::FALSE, //Os dados já estão normalizados, então False para a normalizalção
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, //strinde
            ptr::null(), //offset (posição os os dados começam no buffer)
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::EnableVertexAttribArray(0);
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

                let time_value = start_time.elapsed().as_secs_f32();
                let green_value = (time_value.sin() / 2.0) + 0.5;

                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);


                    gl::UseProgram(shader_program);

                    gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

                    gl::BindVertexArray(vao);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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
