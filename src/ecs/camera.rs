use amethyst::{
    core::{nalgebra::Perspective3, shrev::EventChannel, specs::prelude::*},
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
