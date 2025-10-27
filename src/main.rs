use glfw::{Action, Context, Key, WindowEvent};
//use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
 
    //Inicia o GLFW, delimita a versão e o profile do Opengl (Versão 3 e core profile. A versão
    //pode ser alterada depois mas acredito que não faça muita diferençã)
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core,));


    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "LWA OpengGl", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    //window.set_key_polling(true);
    //window.set_framebuffer_size_polling(true);

    //Carrega os ponteiros das funções do OpenGl
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() {
        window.swap_buffers();

    }
}

