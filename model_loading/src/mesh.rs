use crate::vertex::Vertex;
use crate::texture::Texture;
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
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
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

    fn setup_mesh(&mut self){
        
        // This "unsafe" is needed cuz im calling an external C function (opengl)
        unsafe {
            // Generate Buffers and array
            gl::GenVertexArrays(1, &mut  self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as isize, 
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Vertex Positions
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                (3* mem::size_of::<f32>()) as *const _,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,  
                mem::size_of::<Vertex>() as i32,
                (6* mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(0);
        }   
    }


    pub fn draw(&self, _shader_program: u32) {
        unsafe{
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }

}


















