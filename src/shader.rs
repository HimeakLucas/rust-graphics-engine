use gl;
use std::ffi::CString;
use std::fs; //fylesystem
use std::ptr;

pub struct Shader {
    pub id: gl::types::GLuint,
}

impl Shader {

    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, String> {
        
        let vertex_code = fs::read_to_string(vertex_path)
            .map_err(|e| format!("ERRO::SHADER::FILE_NOT_READ (Vertex): {}", e))?;
        let fragment_code = fs::read_to_string(fragment_path)
            .map_err(|e| format!("ERRO::SHADER::FILE_NOT_READ (Fragment): {}", e))?;

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        unsafe {

            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            Self::check_compile_errors(vertex, "VERTEX")?;

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            Self::check_compile_errors(fragment, "FRAGMENT")?;

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            Self::check_compile_errors(id, "PROGRAM")?;

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            Ok(Shader {id})

        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr()), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &str,  value: f32) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
    }

    unsafe fn check_compile_errors(shader_id: u32, shader_type: &str) -> Result<(), String> {
        unsafe {
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); //subtrai 1 para o terminador nulo \0

            if shader_type != "PROGRAM" {
                gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as gl::types::GLint {
                    gl::GetShaderInfoLog(shader_id, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
                    let error_message = String::from_utf8_lossy(&info_log).to_string();
                    return Err(format!("ERRO::SHADER_COMPILATION_ERROR ({}):\n{}", shader_type, error_message));
                }
            } else {
                gl::GetProgramiv(shader_id, gl::LINK_STATUS, &mut success);
                if success != gl::TRUE as gl::types::GLint {
                    gl::GetProgramInfoLog(shader_id, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
                    let error_message = String::from_utf8_lossy(&info_log).to_string();
                    return Err(format!("ERRO::PROGRAM_LINKING_ERROR ({}):\n{}", shader_type, error_message));
                }
            } 
            Ok(())
        }
    }

}

impl Drop for Shader {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
