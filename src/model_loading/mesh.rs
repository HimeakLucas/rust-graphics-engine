use super::vertex::Vertex;
use super::texture::Texture;

use gl;
use std::mem;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,

    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };

        mesh.setup_mesh();
        mesh
    }

    fn setup_mesh(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            // VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = mem::size_of::<Vertex>() as i32;

            // ---- POSITION (location = 0)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                0 as *const _,
            );

            // ---- NORMAL (location = 1)
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<f32>()) as *const _,
            );

            // ---- TEXCOORD (location = 2)
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<f32>()) as *const _,
            );

            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self, shader_program: u32) {
    unsafe {
        if let Some(tex) = self.textures.first() {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex.id);

            let name = std::ffi::CString::new("texture_diffuse1").unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(shader_program, name.as_ptr()),
                0,
            );
        }

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::BindVertexArray(0);
    }
}

}
