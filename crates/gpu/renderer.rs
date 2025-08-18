use crate::{
    Camera, Scene, Vertex,
    shaders::{FRAGMENT_SHADER, VERTEX_SHADER},
};
use glam::Mat4;
use std::iter;
use utilities::traits::Object;
use wgpu::util::DeviceExt;
use winit::window::Window;

/// Uniforms для передачи MVP
#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}
impl Uniforms {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            mvp: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
    fn from_mat4(m: Mat4) -> Self {
        Self {
            mvp: m.to_cols_array_2d(),
        }
    }
    fn as_byte_slice(uniforms: &[Uniforms]) -> &[u8] {
        let len = std::mem::size_of_val(uniforms);
        unsafe { std::slice::from_raw_parts(uniforms.as_ptr() as *const u8, len) }
    }
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    // Пайплайн и биндинги
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    // MSAA и глубина
    msaa_texture_view: wgpu::TextureView,
    depth_texture_view: wgpu::TextureView,
    sample_count: u32,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("No adapter");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");
        // capabilities / format
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Настройка MSAA
        let sample_count = 4; // 4x MSAA

        // Создание мультисемплированной текстуры
        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let msaa_texture_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Создание текстуры глубины
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // uniform bind group layout (group 0 binding 0)
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform BGL"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        // pipeline layout uses uniform layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        // shader modules
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(VERTEX_SHADER.into()),
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(FRAGMENT_SHADER.into()),
        });
        // render pipeline с поддержкой MSAA и глубины
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            uniform_bind_group_layout,
            msaa_texture_view,
            depth_texture_view,
            sample_count,
        }
    }

    /// Перенастроить surface при ресайзе
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);

            // Пересоздание MSAA текстуры
            let msaa_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MSAA Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: self.sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: self.config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.msaa_texture_view =
                msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Пересоздание текстуры глубины
            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: self.sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.depth_texture_view =
                depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    /// Рендерить сцену с камерой
    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Failed to acquire next surface texture: {:?}", e);
                return;
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Вспомогательная структура для GPU-ресурсов
        struct ObjGpu {
            vertex_buffer: wgpu::Buffer,
            index_buffer: wgpu::Buffer,
            bind_group: wgpu::BindGroup,
            index_count: u32,
        }
        let mut objs_gpu: Vec<ObjGpu> = Vec::with_capacity(scene.objects().len());
        let aspect = self.config.width as f32 / self.config.height as f32;
        let view_mat = camera.view_matrix();
        let proj_mat = camera.projection_matrix(aspect);

        // Подготовка GPU-ресурсов для всех объектов
        for obj in scene.objects() {
            let vertices = obj.vertices();
            let indices = obj.indices();

            // vertex buffer
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Object Vertex Buffer"),
                    contents: Vertex::as_byte_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            // index buffer
            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Object Index Buffer"),
                    contents: unsafe {
                        std::slice::from_raw_parts(
                            indices.as_ptr() as *const u8,
                            std::mem::size_of_val(indices),
                        )
                    },
                    usage: wgpu::BufferUsages::INDEX,
                });

            // uniform MVP = projection * view * model
            let model = obj.model_matrix();
            let mvp = proj_mat * view_mat * model;
            let uniforms = Uniforms::from_mat4(mvp);
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Uniform Buffer"),
                        contents: Uniforms::as_byte_slice(&[uniforms]),
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    });

            let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform Bind Group"),
                layout: &self.uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            objs_gpu.push(ObjGpu {
                vertex_buffer,
                index_buffer,
                bind_group: uniform_bind_group,
                index_count: indices.len() as u32,
            });
        }

        // Создание командного энкодера и прохода рендеринга
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_texture_view, // Рендеринг в MSAA текстуру
                    resolve_target: Some(&view),   // Разрешение в основную текстуру
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.12,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline);

            // Отрисовка всех объектов
            for obj_gpu in objs_gpu.iter() {
                rpass.set_bind_group(0, &obj_gpu.bind_group, &[]);
                rpass.set_vertex_buffer(0, obj_gpu.vertex_buffer.slice(..));
                rpass.set_index_buffer(obj_gpu.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.draw_indexed(0..obj_gpu.index_count, 0, 0..1);
            }
        }

        // Отправка команд и презентация кадра
        self.queue.submit(iter::once(encoder.finish()));
        frame.present();
    }
}
