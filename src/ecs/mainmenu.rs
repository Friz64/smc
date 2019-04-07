use super::CurrentState;
use amethyst::core::{specs::prelude::*, timing::Time, Transform};

#[derive(Default)]
pub struct Rotates;

impl Component for Rotates {
    type Storage = NullStorage<Self>;
}

pub struct MainMenuRotation {
    speed: f32,
}

impl MainMenuRotation {
    pub fn new(speed: f32) -> Self {
        MainMenuRotation { speed }
    }
}

impl<'a> System<'a> for MainMenuRotation {
    type SystemData = (
        ReadExpect<'a, CurrentState>,
        ReadStorage<'a, Rotates>,
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (cur_state, rotates, time, mut transform): Self::SystemData) {
        if *cur_state == CurrentState::MainMenu {
            for (transform, _) in (&mut transform, &rotates).join() {
                transform.yaw_local(self.speed * time.delta_seconds());
            }
        }
    }
}
