use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gpu::{Camera, Mat4, Object3D, Renderer, Scene, Vec3, Vertex};

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
            position: [-0.5, -0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let indices = vec![0u16, 1, 2];

    // Создаём грид (размер 20, шаг 1)
    let grid = Grid::new(10000, 0.5, 0.01);
    let grid_object = Object3D::new(
        grid.vertices().to_vec(),
        grid.indices().to_vec(),
        Mat4::IDENTITY,
    );
    // Можно пометить, что это грид, если у Object3D есть флаг, например:
    // grid_object.set_is_grid(true);
    scene.add_object(grid_object);

    let triangle = Object3D::new(vertices, indices, Mat4::IDENTITY);
    scene.add_object(triangle);

    let mut yaw: f32 = -std::f32::consts::FRAC_PI_2; // смотрим на -Z по умолчанию
    let mut pitch: f32 = 0.0;

    let mut last_mouse_pos = None;
    let sensitivity = 0.002;

    // Камера — смотрит сверху и под углом
    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 2.0), // чуть по диагонали и повыше
        Vec3::ZERO,
        Vec3::Y,
        45.0_f32.to_radians(),
        0.1,
        100.0,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(new_inner_size.width, new_inner_size.height);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if let Some((last_x, last_y)) = last_mouse_pos {
                        let dx = position.x as f32 - last_x;
                        let dy = position.y as f32 - last_y;

                        yaw += dx * sensitivity;
                        pitch -= dy * sensitivity;

                        // Ограничиваем pitch, чтобы не было переворота
                        pitch = pitch.clamp(-89f32.to_radians(), 89f32.to_radians());
                    }
                    last_mouse_pos = Some((position.x as f32, position.y as f32));
                }
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            winit::event::VirtualKeyCode::W => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x, old.y, old.z + 0.5);
                            }
                            winit::event::VirtualKeyCode::S => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x, old.y, old.z - 0.5);
                            }
                            winit::event::VirtualKeyCode::A => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x + 0.5, old.y, old.z);
                            }
                            winit::event::VirtualKeyCode::D => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x - 0.5, old.y, old.z);
                            }
                            winit::event::VirtualKeyCode::LShift => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x, old.y - 0.5, old.z);
                            }
                            winit::event::VirtualKeyCode::Space => {
                                let old = camera.position;
                                camera.position = Vec3::new(old.x, old.y + 0.5, old.z);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let front = Vec3::new(
                    yaw.cos() * pitch.cos(),
                    pitch.sin(),
                    yaw.sin() * pitch.cos(),
                )
                .normalize();

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

        // Горизонтальные линии (по X) - красные
        for i in 0..=size {
            let z = -half + i as f32 * step;
            add_line(
                [-half, 0.0, z],
                [half, 0.0, z],
                2,               // сдвиг по оси Z
                [1.0, 1.0, 1.0], // красный
            );
        }

        // Вертикальные линии (по Z) - зеленые
        for i in 0..=size {
            let x = -half + i as f32 * step;
            add_line(
                [x, 0.0, -half],
                [x, 0.0, half],
                0,               // сдвиг по оси X
                [1.0, 1.0, 1.0], // зеленый
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
