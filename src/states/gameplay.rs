use amethyst::{
    controls::HideCursor,
    core::{shrev::EventChannel, specs::ReaderId},
    input::InputEvent,
    prelude::*,
};

pub struct GameplayState {
    event_reader: Option<ReaderId<InputEvent<String>>>,
    paused: bool,
}

impl GameplayState {
    pub fn new() -> GameplayState {
        GameplayState {
            event_reader: None,
            paused: false,
        }
    }
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        self.event_reader = Some(
            world
                .write_resource::<EventChannel<InputEvent<String>>>()
                .register_reader(),
        );
    }

    fn update(&mut self, StateData { world, .. }: &mut StateData<GameData>) -> SimpleTrans {
        (*world.write_resource::<HideCursor>()).hide = !self.paused;

        Trans::None
    }

    fn shadow_update(&mut self, StateData { world, .. }: StateData<GameData>) {
        let event_channel = world.read_resource::<EventChannel<InputEvent<String>>>();

        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                match &**action {
                    "pause" => self.paused = !self.paused,
                    _ => (),
                }
            }
        }
    }
}
