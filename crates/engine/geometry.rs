use gpu::{Mat4, Object, Object3D, Vertex};

pub struct Triangle {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    model_matrix: Mat4,
}

impl Object for Triangle {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u16] {
        &self.indices
    }

    fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }

    fn to_object3d(self) -> Object3D {
        Object3D::new(
            self.vertices.to_vec(),
            self.indices.to_vec(),
            self.model_matrix,
        )
    }
}

impl Triangle {
    #[allow(clippy::new_without_default)]
    pub fn new(model_matrix: Mat4) -> Self {
        let vertices = vec![
            Vertex {
                position: [-0.5, 0.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];
        let indices = vec![0u16, 1, 2];

        Self {
            vertices,
            indices,
            model_matrix,
        }
    }
}

pub struct Grid {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    model_matrix: Mat4,
}

impl Object for Grid {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u16] {
        &self.indices
    }

    fn to_object3d(self) -> Object3D {
        Object3D::new(
            self.vertices().to_vec(),
            self.indices().to_vec(),
            self.model_matrix,
        )
    }

    fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }
}

impl Grid {
    pub fn new(size: usize, step: f32, line_width: f32, model_matrix: Mat4) -> Self {
        let half = size as f32 * step / 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // helper для добавления двусторонней полоски
        let mut add_line = |start: [f32; 3], end: [f32; 3], perp_axis: usize, color: [f32; 3]| {
            let base_idx = vertices.len() as u16;
            let half_width = line_width / 2.0;

            // Создаем 4 вершины для прямоугольника
            let mut v0 = start;
            v0[perp_axis] -= half_width;
            let mut v1 = end;
            v1[perp_axis] -= half_width;
            let mut v2 = end;
            v2[perp_axis] += half_width;
            let mut v3 = start;
            v3[perp_axis] += half_width;

            for pos in &[v0, v1, v2, v3] {
                vertices.push(Vertex {
                    position: *pos,
                    color,
                });
            }

            // Лицевая сторона (против часовой стрелки)
            indices.extend_from_slice(&[
                base_idx,     // v0
                base_idx + 1, // v1
                base_idx + 2, // v2
                base_idx,     // v0
                base_idx + 2, // v2
                base_idx + 3, // v3
            ]);

            // Обратная сторона (по часовой стрелке)
            indices.extend_from_slice(&[
                base_idx,     // v0
                base_idx + 2, // v2
                base_idx + 1, // v1
                base_idx,     // v0
                base_idx + 3, // v3
                base_idx + 2, // v2
            ]);
        };

        // Горизонтальные линии (по X) - белые
        for i in 0..=size {
            let z = -half + i as f32 * step;
            add_line(
                [-half, 0.0, z],
                [half, 0.0, z],
                2,               // сдвиг по оси Z
                [1.0, 1.0, 1.0], // белый
            );
        }

        // Вертикальные линии (по Z) - белые
        for i in 0..=size {
            let x = -half + i as f32 * step;
            add_line(
                [x, 0.0, -half],
                [x, 0.0, half],
                0,               // сдвиг по оси X
                [1.0, 1.0, 1.0], // белый
            );
        }

        println!(
            "Grid created: {} vertices, {} indices",
            vertices.len(),
            indices.len()
        );

        Self {
            vertices,
            indices,
            model_matrix,
        }
    }
}
