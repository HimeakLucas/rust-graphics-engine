use crate::{mesh::Mesh, vertex::Vertex};
use glam::{Vec2, Vec3};

use std::path::Path;
use tobj;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub directory: String,
}

impl Model {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let (models, _materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        let directory = Path::new(path)
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or("")
            .to_string();

        let mut meshes = Vec::new();

        for model in models {
            let mesh = &model.mesh;

            let mut vertices = Vec::new();
            vertices.reserve(mesh.positions.len() / 3);

            for i in 0..mesh.positions.len() / 3 {
                let pos = Vec3::new(
                    mesh.positions[3 * i],
                    mesh.positions[3 * i + 1],
                    mesh.positions[3 * i + 2],
                );

                let normal = if !mesh.normals.is_empty() {
                    Vec3::new(
                        mesh.normals[3 * i],
                        mesh.normals[3 * i + 1],
                        mesh.normals[3 * i + 2],
                    )
                } else {
                    Vec3::ZERO
                };

                let tex = if !mesh.texcoords.is_empty() {
                    Vec2::new(mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1])
                } else {
                    Vec2::ZERO
                };

                vertices.push(Vertex {
                    position: pos,
                    normal,
                    tex_coords: tex,
                });
            }

            let indices = mesh.indices.clone();
            let textures = vec![];

            meshes.push(Mesh::new(vertices, indices, textures));
        }

        Ok(Self { meshes, directory })
    }

    pub fn draw(&self, shader_program: u32) {
        for mesh in &self.meshes {
            mesh.draw(shader_program);
        }
    }
}
