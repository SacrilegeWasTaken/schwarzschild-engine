use engine::{
    geometry::{Grid, Triangle},
    *,
};
use gpu::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    *,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wgpu Test Scene")
        .build(&event_loop)
        .unwrap();
    let mut engine = SchwarzEngine::new(0.005, window);

    let triangle = Triangle::new(Mat4::IDENTITY).to_object3d();
    let grid = Grid::new(100, 0.5, 0.01, Mat4::IDENTITY).to_object3d();

    engine.add_object_to_scene(grid);
    engine.add_object_to_scene(triangle);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        engine.render(event, control_flow);
    });
}
