use crate::{
    ecs::{mainmenu::Rotates, CurrentState},
    graphics,
    states::{GameplayData, GameplayState},
};
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, ProgressCounter},
    core::{
        nalgebra::{Quaternion, Translation3, UnitQuaternion, Vector3},
        specs::Entity,
        Transform,
    },
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture},
    ui::{UiEventType, UiFinder, UiPrefab},
};

#[derive(Clone)]
pub struct MainMenuData {
    pub mtl: Material,
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
}

impl MainMenuData {
    pub fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> MainMenuData {
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let material = world.exec(|loader: AssetLoaderSystemData<Texture>| Material {
            albedo: loader.load_from_data([0.8, 0.1, 0.1, 1.0].into(), &mut *progress_counter),
            ..material_defaults
        });
        let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
            loader.load("mesh/suzanne.obj", ObjFormat, (), &mut *progress_counter)
        });
        let transform = Transform::default();

        MainMenuData {
            mtl: material,
            mesh,
            transform,
        }
    }
}

pub struct MainMenuState {
    data: MainMenuData,
    mainmenu_gui: Handle<UiPrefab>,
    mainmenu_gui_entity: Option<Entity>,
    suzanne: Option<Entity>,
    light: Option<Entity>,
    camera: Option<Entity>,
    gameplay_data: GameplayData,
    play: Option<Entity>,
    settings: Option<Entity>,
    quit: Option<Entity>,
}

impl MainMenuState {
    pub fn new(
        mainmenu_gui: Handle<UiPrefab>,
        mainmenu_data: MainMenuData,
        gameplay_data: GameplayData,
    ) -> MainMenuState {
        MainMenuState {
            data: mainmenu_data,
            mainmenu_gui,
            mainmenu_gui_entity: None,
            suzanne: None,
            light: None,
            camera: None,
            gameplay_data,
            play: None,
            settings: None,
            quit: None,
        }
    }
}

fn enter(state: &mut MainMenuState, world: &mut World) {
    *world.write_resource::<CurrentState>() = CurrentState::MainMenu;
    state.mainmenu_gui_entity = Some(
        world
            .create_entity()
            .with(state.mainmenu_gui.clone())
            .build(),
    );

    state.suzanne = Some(
        world
            .create_entity()
            .with(Rotates)
            .with(state.data.mtl.clone())
            .with(state.data.mesh.clone())
            .with(state.data.transform.clone())
            .build(),
    );
    state.light = Some(graphics::initialize_light(world));
    state.camera = Some(graphics::initialize_camera(
        world,
        Transform::new(
            Translation3::new(1.0, 0.0, -4.0),
            UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 1.0, 0.0)),
            Vector3::new(1.0, 1.0, 1.0),
        ),
    ));
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        enter(self, world);
    }

    fn on_resume(&mut self, StateData { world, .. }: StateData<GameData>) {
        enter(self, world);
    }

    fn on_pause(&mut self, StateData { world, .. }: StateData<GameData>) {
        if let Some(mainmenu) = self.mainmenu_gui_entity {
            world.delete_entity(mainmenu).unwrap();
        }

        self.play = None;
        self.settings = None;
        self.quit = None;

        world.delete_entity(self.suzanne.unwrap()).unwrap();
        world.delete_entity(self.light.unwrap()).unwrap();
        world.delete_entity(self.camera.unwrap()).unwrap();
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(ui_event) => {
                if ui_event.event_type == UiEventType::ClickStop {
                    if matches(ui_event.target, self.play) {
                        Trans::Push(Box::new(GameplayState::new(self.gameplay_data.clone())))
                    } else if matches(ui_event.target, self.settings) {
                        Trans::None
                    } else if matches(ui_event.target, self.quit) {
                        Trans::Quit
                    } else {
                        Trans::None
                    }
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, StateData { world, .. }: &mut StateData<GameData>) -> SimpleTrans {
        if self.play.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("play") {
                    self.play = Some(entity);
                }
            });
        }
        if self.settings.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("settings") {
                    self.settings = Some(entity);
                }
            });
        }
        if self.quit.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("quit") {
                    self.quit = Some(entity);
                }
            });
        }

        Trans::None
    }
}

fn matches(target: Entity, entity: Option<Entity>) -> bool {
    match (target, entity) {
        (target, Some(entity)) if target == entity => true,
        _ => false,
    }
}
