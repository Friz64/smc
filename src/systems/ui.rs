use amethyst::{
    core::{shrev::EventChannel, specs::prelude::*},
    ui::UiEvent,
};

pub struct UiEventHandler {
    event_reader: Option<ReaderId<UiEvent>>,
}

impl UiEventHandler {
    pub fn new() -> Self {
        UiEventHandler { event_reader: None }
    }
}

impl<'a> System<'a> for UiEventHandler {
    type SystemData = Read<'a, EventChannel<UiEvent>>;

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(res.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }

    fn run(&mut self, events: Self::SystemData) {
        for event in events.read(self.event_reader.as_mut().unwrap()) {
            println!("{:?}", event);
        }
    }
}
