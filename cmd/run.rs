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

    let triangle = Triangle::new(Mat4::IDENTITY);
    let grid = Grid::new(100, 0.5, 0.01, Mat4::IDENTITY);

    let mut obj = utilities::obj_import::load_obj("resources/mercedes_ponos.obj", Mat4::IDENTITY)
        .expect("obj load error");

    obj.translate(Vec3::new(10f32, 0f32, 10f32));
    obj.scale(Vec3::new(0.33, 0.33, 0.33));

    engine.add_object_to_scene(grid);
    engine.add_object_to_scene(triangle);
    engine.add_object_to_scene(obj);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        engine.render(event, control_flow);
    });
}
