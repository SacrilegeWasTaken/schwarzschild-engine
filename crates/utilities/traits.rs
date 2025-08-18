use glam::{Mat4, Vec3};

use crate::common::{Object3D, Vertex};

/// Трейт объекта сцены
pub trait Object {
    /// вершины в локальных координатах
    fn vertices(&self) -> &[Vertex];
    /// индексы (триангулированные, u16)
    fn indices(&self) -> &[u32];
    /// модельная матрица (локальная -> мировая)
    fn model_matrix(&self) -> Mat4;

    fn to_object3d(self) -> Object3D;

    fn translate(&mut self, offset: Vec3); /*{
    self.model_matrix = Mat4::from_translation(offset) * self.model_matrix;
    } */

    /// масштабировать объект
    fn scale(&mut self, factor: Vec3); /*{
    self.model_matrix = Mat4::from_scale(factor) * self.model_matrix;
    }*/
}
