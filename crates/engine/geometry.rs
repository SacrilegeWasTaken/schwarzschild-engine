use gpu::{Mat4, Object, Object3D, Vertex};

pub struct Triangle {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    model_matrix: Mat4,
}

impl Object for Triangle {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn indices(&self) -> &[u32] {
        &self.indices
    }
    fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }

    fn to_object3d(self) -> Object3D {
        Object3D::new(
            self.vertices.clone(),
            self.indices.clone(),
            self.model_matrix,
        )
    }

    fn translate(&mut self, offset: glam::Vec3) {
        self.model_matrix = Mat4::from_translation(offset) * self.model_matrix;
    }

    fn scale(&mut self, factor: glam::Vec3) {
        self.model_matrix = Mat4::from_scale(factor) * self.model_matrix;
    }
}

impl Triangle {
    pub fn new(model_matrix: Mat4) -> Self {
        // позиции вершин
        let positions = [[-0.5, 0.0, 0.0], [0.5, 0.0, 0.0], [0.0, 1.0, 0.0]];

        // вычисляем нормаль
        let a = glam::Vec3::from(positions[0]);
        let b = glam::Vec3::from(positions[1]);
        let c = glam::Vec3::from(positions[2]);
        let normal = (b - a).cross(c - a).normalize().to_array();

        // собираем вершины
        let vertices = vec![
            Vertex {
                position: positions[0],
                normal,
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: positions[1],
                normal,
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: positions[2],
                normal,
                color: [0.0, 0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2];

        Self {
            vertices,
            indices,
            model_matrix,
        }
    }
}

pub struct Grid {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    model_matrix: Mat4,
}

impl Object for Grid {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn indices(&self) -> &[u32] {
        &self.indices
    }
    fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }

    fn to_object3d(self) -> Object3D {
        Object3D::new(
            self.vertices.clone(),
            self.indices.clone(),
            self.model_matrix,
        )
    }

    fn translate(&mut self, offset: glam::Vec3) {
        self.model_matrix = Mat4::from_translation(offset) * self.model_matrix;
    }

    fn scale(&mut self, factor: glam::Vec3) {
        self.model_matrix = Mat4::from_scale(factor) * self.model_matrix;
    }
}

impl Grid {
    pub fn new(size: usize, step: f32, line_width: f32, model_matrix: Mat4) -> Self {
        let half = size as f32 * step / 2.0;
        let mut vertices = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut add_line = |start: [f32; 3], end: [f32; 3], perp_axis: usize, color: [f32; 3]| {
            let base_idx = vertices.len() as u32;
            let half_width = line_width / 2.0;

            // четыре точки линии
            let mut v0 = start;
            v0[perp_axis] -= half_width;
            let mut v1 = end;
            v1[perp_axis] -= half_width;
            let mut v2 = end;
            v2[perp_axis] += half_width;
            let mut v3 = start;
            v3[perp_axis] += half_width;

            let normal_front = [0.0, 1.0, 0.0];
            let normal_back = [0.0, -1.0, 0.0];

            // front
            for pos in &[v0, v1, v2, v3] {
                vertices.push(Vertex {
                    position: *pos,
                    normal: normal_front,
                    color,
                });
            }
            // back
            for pos in &[v0, v1, v2, v3] {
                vertices.push(Vertex {
                    position: *pos,
                    normal: normal_back,
                    color,
                });
            }

            // индексы для front (0..3)
            indices.extend_from_slice(&[
                base_idx,
                base_idx + 1,
                base_idx + 2,
                base_idx,
                base_idx + 2,
                base_idx + 3,
            ]);

            // индексы для back (4..7)
            let b = base_idx + 4;
            indices.extend_from_slice(&[b, b + 2, b + 1, b, b + 3, b + 2]);
        };

        // линии X
        for i in 0..=size {
            let z = -half + i as f32 * step;
            add_line([-half, 0.0, z], [half, 0.0, z], 2, [1.0, 1.0, 1.0]);
        }

        // линии Z
        for i in 0..=size {
            let x = -half + i as f32 * step;
            add_line([x, 0.0, -half], [x, 0.0, half], 0, [1.0, 1.0, 1.0]);
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
