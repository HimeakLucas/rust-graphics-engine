use cgmath::Vector3;
use crate::shader::Shader;

pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material {

    pub fn new(
        ambient: Vector3<f32>,
        diffuse: Vector3<f32>,
        specular: Vector3<f32>,
        shininess: f32
    ) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn apply(&self, shader: &Shader, uniform_name: &str) {
        shader.set_vec3(&format!("{}.ambient", uniform_name), &self.ambient);
        shader.set_vec3(&format!("{}.diffuse", uniform_name), &self.diffuse);
        shader.set_vec3(&format!("{}.specular", uniform_name), &self.specular);
        shader.set_float(&format!("{}.shininess", uniform_name), self.shininess);
    }
}
