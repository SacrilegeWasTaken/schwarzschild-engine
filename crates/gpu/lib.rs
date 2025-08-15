mod renderer;
mod shaders;
mod vertex;

pub use glam::*;
pub use pollster::*;
pub use renderer::Renderer;
pub use shaders::{FRAGMENT_SHADER, VERTEX_SHADER};
pub use vertex::Vertex;
pub use winit::*;

/// Трейт объекта сцены
pub trait Object {
    /// вершины в локальных координатах
    fn vertices(&self) -> &[Vertex];
    /// индексы (триангулированные, u16)
    fn indices(&self) -> &[u16];
    /// модельная матрица (локальная -> мировая)
    fn model_matrix(&self) -> Mat4;

    fn to_object3d(self) -> Object3D;
}

pub struct Object3D {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    model_matrix: Mat4,
}

impl Object3D {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>, model_matrix: Mat4) -> Self {
        Self {
            vertices,
            indices,
            model_matrix,
        }
    }
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }
    pub fn model_matrix(&self) -> Mat4 {
        self.model_matrix
    }
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object3D>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_object(&mut self, obj: Object3D) {
        self.objects.push(obj);
    }
    pub fn objects(&self) -> &[Object3D] {
        &self.objects
    }
}
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov: f32, near: f32, far: f32) -> Self {
        Self {
            position,
            target,
            up,
            fov,
            near,
            far,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov, aspect_ratio, self.near, self.far)
    }
}
