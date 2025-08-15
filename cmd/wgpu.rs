use gpu::{Camera, Mat4, Object3D, Renderer, Scene, Vec3, Vertex};
use std::time::Instant;
use winit::{
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu Test Scene")
        .build(&event_loop)
        .unwrap();
    let mut renderer = pollster::block_on(Renderer::new(&window));

    // Создаём сцену
    let mut scene = Scene::new();

    // Простой треугольник
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

    // Создаём грид
    let grid = Grid::new(100, 0.5, 0.01);
    let grid_object = Object3D::new(
        grid.vertices().to_vec(),
        grid.indices().to_vec(),
        Mat4::IDENTITY,
    );
    scene.add_object(grid_object);

    let triangle = Object3D::new(vertices, indices, Mat4::IDENTITY);
    scene.add_object(triangle);

    let mut yaw: f32 = -std::f32::consts::FRAC_PI_2; // смотрим на -Z по умолчанию
    let mut pitch: f32 = 0.0;
    let sensitivity = 0.002;

    // Параметры движения камеры
    let move_speed = 5.0; // Максимальная скорость движения
    let acceleration_time = 0.25; // Время разгона/торможения в секундах
    let acceleration = move_speed / acceleration_time; // Ускорение

    // Текущие скорости по осям
    let mut velocity_forward = 0.0;
    let mut velocity_right = 0.0;
    let mut velocity_up = 0.0;

    // Флаги движения
    let mut moving_forward = false;
    let mut moving_backward = false;
    let mut moving_left = false;
    let mut moving_right = false;
    let mut moving_up = false;
    let mut moving_down = false;

    // Флаг захвата мыши
    let mut mouse_captured = false;

    // Камера
    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 2.0),
        Vec3::ZERO,
        Vec3::Y,
        45.0_f32.to_radians(),
        0.1,
        100.0,
    );

    // Для отслеживания времени
    let mut last_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // Обработка событий устройства (мышь)
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if mouse_captured {
                    yaw += delta.0 as f32 * sensitivity;
                    pitch -= delta.1 as f32 * sensitivity;
                    pitch = pitch.clamp(-89f32.to_radians(), 89f32.to_radians());
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(new_inner_size.width, new_inner_size.height);
                }
                // Обработка нажатия/отпускания кнопок мыши
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        mouse_captured = state == ElementState::Pressed;
                    }
                }
                // Обработка нажатия/отпускания клавиш
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            winit::event::VirtualKeyCode::W => {
                                moving_forward = input.state == ElementState::Pressed;
                            }
                            winit::event::VirtualKeyCode::S => {
                                moving_backward = input.state == ElementState::Pressed;
                            }
                            winit::event::VirtualKeyCode::A => {
                                moving_left = input.state == ElementState::Pressed;
                            }
                            winit::event::VirtualKeyCode::D => {
                                moving_right = input.state == ElementState::Pressed;
                            }
                            winit::event::VirtualKeyCode::Space => {
                                moving_up = input.state == ElementState::Pressed;
                            }
                            winit::event::VirtualKeyCode::LShift => {
                                moving_down = input.state == ElementState::Pressed;
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
                let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
                last_frame_time = current_time;

                // Обновляем скорости на основе нажатых клавиш
                // Вперёд/назад
                if moving_forward {
                    velocity_forward =
                        (velocity_forward + acceleration * delta_time).min(move_speed);
                } else if moving_backward {
                    velocity_forward =
                        (velocity_forward - acceleration * delta_time).max(-move_speed);
                } else {
                    // Плавное замедление
                    if velocity_forward > 0.0 {
                        velocity_forward = (velocity_forward - acceleration * delta_time).max(0.0);
                    } else {
                        velocity_forward = (velocity_forward + acceleration * delta_time).min(0.0);
                    }
                }

                // Влево/вправо
                if moving_right {
                    velocity_right = (velocity_right + acceleration * delta_time).min(move_speed);
                } else if moving_left {
                    velocity_right = (velocity_right - acceleration * delta_time).max(-move_speed);
                } else {
                    // Плавное замедление
                    if velocity_right > 0.0 {
                        velocity_right = (velocity_right - acceleration * delta_time).max(0.0);
                    } else {
                        velocity_right = (velocity_right + acceleration * delta_time).min(0.0);
                    }
                }

                // Вверх/вниз
                if moving_up {
                    velocity_up = (velocity_up + acceleration * delta_time).min(move_speed);
                } else if moving_down {
                    velocity_up = (velocity_up - acceleration * delta_time).max(-move_speed);
                } else {
                    // Плавное замедление
                    if velocity_up > 0.0 {
                        velocity_up = (velocity_up - acceleration * delta_time).max(0.0);
                    } else {
                        velocity_up = (velocity_up + acceleration * delta_time).min(0.0);
                    }
                }

                // Вычисляем векторы направления камеры
                let front = Vec3::new(
                    yaw.cos() * pitch.cos(),
                    pitch.sin(),
                    yaw.sin() * pitch.cos(),
                )
                .normalize();
                let right = front.cross(Vec3::Y).normalize();
                let up = right.cross(front).normalize();

                // Обновляем позицию камеры на основе текущих скоростей
                camera.position += front * velocity_forward * delta_time;
                camera.position += right * velocity_right * delta_time;
                camera.position += up * velocity_up * delta_time;

                // Обновляем цель камеры
                let target = camera.position + front;
                camera.target = target;

                renderer.render(&scene, &camera);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

// Код Grid остается без изменений
pub struct Grid {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Grid {
    pub fn new(size: usize, step: f32, line_width: f32) -> Self {
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

        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }
}
