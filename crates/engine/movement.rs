pub(crate) struct CameraMovement {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,

    // Параметры движения камеры
    pub move_speed: f32,
    pub acceleration_time: f32,
    pub acceleration: f32,

    // Текущие скорости по осям
    pub velocity_forward: f32,
    pub velocity_right: f32,
    pub velocity_up: f32,

    // Флаги движения
    pub moving_forward: bool,
    pub moving_backward: bool,
    pub moving_left: bool,
    pub moving_right: bool,
    pub moving_up: bool,
    pub moving_down: bool,

    // Флаг захвата мыши
    pub mouse_captured: bool,
}

impl CameraMovement {
    pub fn new(sensitivity: f32) -> Self {
        let yaw: f32 = -std::f32::consts::FRAC_PI_2;
        let pitch: f32 = 0.0;

        // Параметры движения камеры
        let move_speed = 5.0; // Максимальная скорость движения
        let acceleration_time = 0.25; // Время разгона/торможения в секундах
        let acceleration = move_speed / acceleration_time; // Ускорение

        // Текущие скорости по осям
        let velocity_forward = 0.0;
        let velocity_right = 0.0;
        let velocity_up = 0.0;

        // Флаги движения
        let moving_forward = false;
        let moving_backward = false;
        let moving_left = false;
        let moving_right = false;
        let moving_up = false;
        let moving_down = false;

        // Флаг захвата мыши
        let mouse_captured = false;
        Self {
            yaw,
            pitch,
            sensitivity,
            move_speed,
            acceleration_time,
            acceleration,
            velocity_forward,
            velocity_right,
            velocity_up,
            moving_forward,
            moving_backward,
            moving_left,
            moving_right,
            moving_up,
            moving_down,
            mouse_captured,
        }
    }
}
