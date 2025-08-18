use tobj::{self, LoadError};

use crate::common::{Object3D, Vertex};

use glam::Mat4;

pub fn load_obj(path: &str, model_matrix: Mat4) -> Result<Vec<Object3D>, LoadError> {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let mut objects = Vec::new();

    // Пока берём только первый меш

    for model in models {
        // Собираем вершины
        let mesh = model.mesh;
        let mut vertices = Vec::with_capacity(mesh.positions.len() / 3);
        for i in 0..mesh.positions.len() / 3 {
            let px = mesh.positions[i * 3];
            let py = mesh.positions[i * 3 + 1];
            let pz = mesh.positions[i * 3 + 2];

            let nx = mesh.normals[i * 3];
            let ny = mesh.normals[i * 3 + 1];
            let nz = mesh.normals[i * 3 + 2];

            // пока просто зададим цвет = позиция (для теста)
            let color = [px.abs(), py.abs(), pz.abs()];

            vertices.push(Vertex {
                position: [px, py, pz],
                color,
                normal: [nx, ny, nz],
            });
        }

        // Индексы (tobj даёт u32)
        let indices: Vec<u32> = mesh.indices;

        let object = Object3D::new(vertices, indices, model_matrix);
        objects.push(object);
    }

    Ok(objects)
}
