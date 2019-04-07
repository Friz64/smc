use super::CurrentState;
use amethyst::{
    controls::{HideCursor, WindowFocus},
    core::{
        nalgebra::{Unit, Vector3},
        shrev::EventChannel,
        specs::prelude::*,
        timing::Time,
        Transform,
    },
    input::{InputEvent, InputHandler},
    renderer::Camera,
};

pub struct CameraRotation {
    event_reader: Option<ReaderId<InputEvent<String>>>,
    sensitivity_x: f32,
    sensitivity_y: f32,
}

impl CameraRotation {
    pub fn new(sensitivity_x: f32, sensitivity_y: f32) -> Self {
        CameraRotation {
            event_reader: None,
            sensitivity_x,
            sensitivity_y,
        }
    }
}

impl<'a> System<'a> for CameraRotation {
    type SystemData = (
        ReadExpect<'a, CurrentState>,
        Read<'a, EventChannel<InputEvent<String>>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<String>>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (cur_state, events, mut transform, camera, focus, hide): Self::SystemData) {
        for event in events.read(self.event_reader.as_mut().unwrap()) {
            if focus.is_focused && hide.hide && *cur_state == CurrentState::Gameplay {
                if let InputEvent::MouseMoved { delta_x, delta_y } = *event {
                    for (transform, _) in (&mut transform, &camera).join() {
                        transform.pitch_local((-delta_y as f32 * self.sensitivity_y).to_radians());
                        transform.yaw_global((-delta_x as f32 * self.sensitivity_x).to_radians());
                    }
                }
            }
        }
    }
}

pub struct CameraMovement {
    speed: f32,
}

impl CameraMovement {
    pub fn new(speed: f32) -> Self {
        CameraMovement { speed }
    }
}

impl<'a> System<'a> for CameraMovement {
    type SystemData = (
        ReadExpect<'a, CurrentState>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn run(
        &mut self,
        (cur_state, time, input_handler, mut transform, camera, focus, hide): Self::SystemData,
    ) {
        if *cur_state == CurrentState::Gameplay {
            let walk = input_handler.axis_value("walk").unwrap() as f32;
            let strafe = input_handler.axis_value("strafe").unwrap() as f32;

            if focus.is_focused && hide.hide {
                if let Some(dir) = Unit::try_new(Vector3::new(-strafe, 0.0, -walk), 1.0e-6) {
                    for (transform, _) in (&mut transform, &camera).join() {
                        transform.move_along_local(dir, time.delta_seconds() * self.speed);
                    }
                }
            }
        }
    }
}
