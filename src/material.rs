use cgmath::Vector3;
use crate::shader::Shader;

pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
    pub texture_unit: Option<i32>, // Se for Some(0), usa textura. Se for None, usa cor.
}

impl Material {
    // Construtor genérico
    pub fn new(
        ambient: Vector3<f32>, 
        diffuse: Vector3<f32>, 
        specular: Vector3<f32>, 
        shininess: f32,
        texture_unit: Option<i32> 
    ) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            texture_unit,
        }
    }

    pub fn apply(&self, shader: &Shader, uniform_name: &str) {
        shader.set_vec3(&format!("{}.specular", uniform_name), &self.specular);
        shader.set_float(&format!("{}.shininess", uniform_name), self.shininess);

        match self.texture_unit {
            Some(unit) => {
                // Ativa modo TEXTURA no shader
                shader.set_int(&format!("{}.use_texture", uniform_name), 1);
                shader.set_int(&format!("{}.texture_diffuse", uniform_name), unit);
                // Ignora ambient/diffuse sólidos
            },
            None => {
                // Ativa modo COR SÓLIDA no shader
                shader.set_int(&format!("{}.use_texture", uniform_name), 0);
                shader.set_vec3(&format!("{}.ambient", uniform_name), &self.ambient);
                shader.set_vec3(&format!("{}.diffuse", uniform_name), &self.diffuse);
            }
        }
    }
}
