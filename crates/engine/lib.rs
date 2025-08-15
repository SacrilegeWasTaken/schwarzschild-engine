pub mod geometry;
mod movement;

use std::time::Instant;

use gpu::{
    Camera, Object, Object3D, Renderer, Scene, Vec3,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::movement::CameraMovement;
pub use winit::*;

pub struct SchwarzEngine {
    scene: Scene,
    window: Window,
    renderer: Renderer,
    camera_movement: CameraMovement,
    camera: Camera,
    last_frame_time: Instant,
}

impl SchwarzEngine {
    #[allow(clippy::new_without_default)]
    pub fn new(sensitivity: f32, window: Window) -> SchwarzEngine {
        let renderer = gpu::block_on(Renderer::new(&window));

        let scene = Scene::new();

        // Камера
        let camera = Camera::new(
            Vec3::new(5.0, 5.0, 2.0),
            Vec3::ZERO,
            Vec3::Y,
            45.0_f32.to_radians(),
            0.1,
            100.0,
        );

        Self {
            scene,
            window,
            renderer,
            camera_movement: CameraMovement::new(sensitivity),
            last_frame_time: Instant::now(),
            camera,
        }
    }

    pub fn add_object_to_scene(&mut self, obj: impl Object) {
        self.scene.add_object(obj.to_object3d());
    }

    pub fn render<T>(&mut self, event: Event<'_, T>, control_flow: &mut ControlFlow) {
        // *control_flow = ControlFlow::Wait;

        match event {
            // Обработка событий устройства (мышь)
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if self.camera_movement.mouse_captured {
                    self.camera_movement.yaw += delta.0 as f32 * self.camera_movement.sensitivity;
                    self.camera_movement.pitch -= delta.1 as f32 * self.camera_movement.sensitivity;
                    self.camera_movement.pitch = self
                        .camera_movement
                        .pitch
                        .clamp(-89f32.to_radians(), 89f32.to_radians());
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    self.renderer.resize(size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.renderer
                        .resize(new_inner_size.width, new_inner_size.height);
                }
                // Обработка нажатия/отпускания кнопок мыши
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        self.camera_movement.mouse_captured = state == ElementState::Pressed;
                    }
                }
                // Обработка нажатия/отпускания клавиш
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            event::VirtualKeyCode::W => {
                                self.camera_movement.moving_forward =
                                    input.state == ElementState::Pressed;
                            }
                            event::VirtualKeyCode::S => {
                                self.camera_movement.moving_backward =
                                    input.state == ElementState::Pressed;
                            }
                            event::VirtualKeyCode::A => {
                                self.camera_movement.moving_left =
                                    input.state == ElementState::Pressed;
                            }
                            event::VirtualKeyCode::D => {
                                self.camera_movement.moving_right =
                                    input.state == ElementState::Pressed;
                            }
                            event::VirtualKeyCode::Space => {
                                self.camera_movement.moving_up =
                                    input.state == ElementState::Pressed;
                            }
                            event::VirtualKeyCode::LShift => {
                                self.camera_movement.moving_down =
                                    input.state == ElementState::Pressed;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Вычисляем время, прошедшее с последнего кадра
                let current_time = Instant::now();
                let delta_time = current_time
                    .duration_since(self.last_frame_time)
                    .as_secs_f32();
                self.last_frame_time = current_time;

                // Обновляем скорости на основе нажатых клавиш
                // Вперёд/назад
                if self.camera_movement.moving_forward {
                    self.camera_movement.velocity_forward = (self.camera_movement.velocity_forward
                        + self.camera_movement.acceleration * delta_time)
                        .min(self.camera_movement.move_speed);
                } else if self.camera_movement.moving_backward {
                    self.camera_movement.velocity_forward = (self.camera_movement.velocity_forward
                        - self.camera_movement.acceleration * delta_time)
                        .max(-self.camera_movement.move_speed);
                } else {
                    // Плавное замедление
                    if self.camera_movement.velocity_forward > 0.0 {
                        self.camera_movement.velocity_forward =
                            (self.camera_movement.velocity_forward
                                - self.camera_movement.acceleration * delta_time)
                                .max(0.0);
                    } else {
                        self.camera_movement.velocity_forward =
                            (self.camera_movement.velocity_forward
                                + self.camera_movement.acceleration * delta_time)
                                .min(0.0);
                    }
                }

                // Влево/вправо
                if self.camera_movement.moving_right {
                    self.camera_movement.velocity_right = (self.camera_movement.velocity_right
                        + self.camera_movement.acceleration * delta_time)
                        .min(self.camera_movement.move_speed);
                } else if self.camera_movement.moving_left {
                    self.camera_movement.velocity_right = (self.camera_movement.velocity_right
                        - self.camera_movement.acceleration * delta_time)
                        .max(-self.camera_movement.move_speed);
                } else {
                    // Плавное замедление
                    if self.camera_movement.velocity_right > 0.0 {
                        self.camera_movement.velocity_right = (self.camera_movement.velocity_right
                            - self.camera_movement.acceleration * delta_time)
                            .max(0.0);
                    } else {
                        self.camera_movement.velocity_right = (self.camera_movement.velocity_right
                            + self.camera_movement.acceleration * delta_time)
                            .min(0.0);
                    }
                }

                // Вверх/вниз
                if self.camera_movement.moving_up {
                    self.camera_movement.velocity_up = (self.camera_movement.velocity_up
                        + self.camera_movement.acceleration * delta_time)
                        .min(self.camera_movement.move_speed);
                } else if self.camera_movement.moving_down {
                    self.camera_movement.velocity_up = (self.camera_movement.velocity_up
                        - self.camera_movement.acceleration * delta_time)
                        .max(-self.camera_movement.move_speed);
                } else {
                    // Плавное замедление
                    if self.camera_movement.velocity_up > 0.0 {
                        self.camera_movement.velocity_up = (self.camera_movement.velocity_up
                            - self.camera_movement.acceleration * delta_time)
                            .max(0.0);
                    } else {
                        self.camera_movement.velocity_up = (self.camera_movement.velocity_up
                            + self.camera_movement.acceleration * delta_time)
                            .min(0.0);
                    }
                }

                // Вычисляем векторы направления камеры
                let front = Vec3::new(
                    self.camera_movement.yaw.cos() * self.camera_movement.pitch.cos(),
                    self.camera_movement.pitch.sin(),
                    self.camera_movement.yaw.sin() * self.camera_movement.pitch.cos(),
                )
                .normalize();
                let right = front.cross(Vec3::Y).normalize();
                let up = right.cross(front).normalize();

                // Обновляем позицию камеры на основе текущих скоростей
                self.camera.position += front * self.camera_movement.velocity_forward * delta_time;
                self.camera.position += right * self.camera_movement.velocity_right * delta_time;
                self.camera.position += up * self.camera_movement.velocity_up * delta_time;

                // Обновляем цель камеры
                let target = self.camera.position + front;
                self.camera.target = target;

                self.renderer.render(&self.scene, &self.camera);
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        }
    }
}
