use glam::Mat4;

use crate::common::{Object3D, Vertex};

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
