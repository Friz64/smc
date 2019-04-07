use crate::{ecs::CurrentState, graphics};
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, ProgressCounter},
    controls::HideCursor,
    core::{
        nalgebra::{Quaternion, Translation3, UnitQuaternion, Vector3},
        shrev::EventChannel,
        specs::prelude::*,
        Transform,
    },
    input::InputEvent,
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture},
};

#[derive(Clone)]
pub struct GameplayData {
    pub mtl: Material,
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
}

impl GameplayData {
    pub fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> GameplayData {
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let material = world.exec(|loader: AssetLoaderSystemData<Texture>| Material {
            albedo: loader.load_from_data([0.0, 0.0, 1.0, 1.0].into(), &mut *progress_counter),
            ..material_defaults
        });
        let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
            loader.load("mesh/suzanne.obj", ObjFormat, (), &mut *progress_counter)
        });
        let transform = Transform::default();

        GameplayData {
            mtl: material,
            mesh,
            transform,
        }
    }
}

pub struct GameplayState {
    data: GameplayData,
    event_reader: Option<ReaderId<InputEvent<String>>>,
    paused: bool,
    suzanne: Option<Entity>,
    light: Option<Entity>,
    camera: Option<Entity>,
}

impl GameplayState {
    pub fn new(data: GameplayData) -> GameplayState {
        GameplayState {
            data,
            event_reader: None,
            paused: false,
            suzanne: None,
            light: None,
            camera: None,
        }
    }
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        *world.write_resource::<CurrentState>() = CurrentState::Gameplay;
        (*world.write_resource::<HideCursor>()).hide = true;

        self.event_reader = Some(
            world
                .write_resource::<EventChannel<InputEvent<String>>>()
                .register_reader(),
        );

        self.suzanne = Some(
            world
                .create_entity()
                .with(self.data.mtl.clone())
                .with(self.data.mesh.clone())
                .with(self.data.transform.clone())
                .build(),
        );
        self.light = Some(graphics::initialize_light(world));
        self.camera = Some(graphics::initialize_camera(
            world,
            Transform::new(
                Translation3::new(0.0, 0.0, -4.0),
                UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 1.0, 0.0)),
                Vector3::new(1.0, 1.0, 1.0),
            ),
        ));
    }

    fn on_stop(&mut self, StateData { world, .. }: StateData<GameData>) {
        (*world.write_resource::<HideCursor>()).hide = false;

        world.delete_entity(self.suzanne.unwrap()).unwrap();
        world.delete_entity(self.light.unwrap()).unwrap();
        world.delete_entity(self.camera.unwrap()).unwrap();
    }

    fn update(&mut self, _: &mut StateData<GameData>) -> SimpleTrans {
        if self.paused {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn shadow_update(&mut self, StateData { world, .. }: StateData<GameData>) {
        let event_channel = world.read_resource::<EventChannel<InputEvent<String>>>();

        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                #[allow(clippy::single_match)]
                match &**action {
                    "pause" => self.paused = !self.paused,
                    _ => (),
                }
            }
        }
    }
}
