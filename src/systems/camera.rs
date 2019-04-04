use super::Gameplay;
use amethyst::{
    controls::{HideCursor, WindowFocus},
    core::{
        nalgebra::Perspective3,
        nalgebra::{Unit, Vector3},
        shrev::EventChannel,
        specs::prelude::*,
        timing::Time,
        Transform,
    },
    input::{InputEvent, InputHandler},
    renderer::{Camera, Event, WindowEvent},
};

// Updates the Camera with a new Projection Matrix to fit the Screen
pub struct CameraAspect {
    event_reader: Option<ReaderId<Event>>,
}

impl CameraAspect {
    pub fn new() -> Self {
        CameraAspect { event_reader: None }
    }
}

impl<'a> System<'a> for CameraAspect {
    type SystemData = (WriteStorage<'a, Camera>, Read<'a, EventChannel<Event>>);

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }

    fn run(&mut self, (mut cameras, event_channel): Self::SystemData) {
        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } = event
            {
                let (width, height): (u32, u32) = (*size).into();
                for camera in (&mut cameras).join() {
                    let mut projection = Perspective3::from_matrix_unchecked(camera.proj);
                    projection.set_aspect(width as f32 / height as f32);

                    camera.proj = projection.to_homogeneous();
                }
            }
        }
    }
}

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
        ReadExpect<'a, Gameplay>,
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

    fn run(&mut self, (gameplay, events, mut transform, camera, focus, hide): Self::SystemData) {
        for event in events.read(self.event_reader.as_mut().unwrap()) {
            if focus.is_focused && hide.hide && gameplay.0 {
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
        ReadExpect<'a, Gameplay>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn run(
        &mut self,
        (gameplay, time, input_handler, mut transform, camera, focus, hide): Self::SystemData,
    ) {
        if gameplay.0 {
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
