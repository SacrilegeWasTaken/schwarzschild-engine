use tobj::{self, LoadError};

use crate::common::{Object3D, Vertex};

use glam::Mat4;

pub fn load_obj(path: &str, model_matrix: Mat4) -> Result<Object3D, LoadError> {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    // Пока берём только первый меш
    let mesh = &models[0].mesh;

    // Собираем вершины
    let mut vertices = Vec::with_capacity(mesh.positions.len() / 3);
    for i in 0..mesh.positions.len() / 3 {
        let px = mesh.positions[i * 3];
        let py = mesh.positions[i * 3 + 1];
        let pz = mesh.positions[i * 3 + 2];

        // пока просто зададим цвет = позиция (для теста)
        let color = [px.abs(), py.abs(), pz.abs()];

        vertices.push(Vertex {
            position: [px, py, pz],
            color,
        });
    }

    // Индексы (tobj даёт u32, конвертируем в u16 если влезают)
    let indices: Vec<u16> = mesh.indices.iter().map(|&i| i as u16).collect();

    Ok(Object3D::new(vertices, indices, model_matrix))
}
