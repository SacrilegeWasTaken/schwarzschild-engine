use std::mem::size_of_val;

use glam::Mat4;

use crate::traits::Object;

/// Описание вершины для 3D: позиция (vec3) + цвет (vec3)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position @location(0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal @location(1)
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color @location(2)
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    /// Unsafe but common: получить &[u8] из слайса вершин
    pub fn as_byte_slice(vertices: &[Vertex]) -> &[u8] {
        let byte_len = size_of_val(vertices);
        unsafe { std::slice::from_raw_parts(vertices.as_ptr() as *const u8, byte_len) }
    }
}

pub struct Object3D {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    model_matrix: Mat4,
}

impl Object3D {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, model_matrix: Mat4) -> Self {
        Self {
            vertices,
            indices,
            model_matrix,
        }
    }
}

impl Object for Object3D {
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
        self
    }

    fn translate(&mut self, offset: glam::Vec3) {
        self.model_matrix = Mat4::from_translation(offset) * self.model_matrix;
    }

    fn scale(&mut self, factor: glam::Vec3) {
        self.model_matrix = Mat4::from_scale(factor) * self.model_matrix;
    }
}
