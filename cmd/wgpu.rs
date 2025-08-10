use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gpu::Renderer;

fn main() {
    // Инициализируем event loop и окно
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu Window")
        .build(&event_loop)
        .unwrap();

    // Создаем рендерер, связываем с окном
    let mut renderer = pollster::block_on(Renderer::new(&window));

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
                _ => {}
            },
            Event::RedrawRequested(_) => {
                renderer.render();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
