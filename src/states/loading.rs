use crate::{graphics, logger::prelude::*, states::GameplayState, ui::CustomUi};
use amethyst::{
    assets::{AssetLoaderSystemData, Completion, ProgressCounter},
    audio::AudioFormat,
    core::{specs::Entity, Transform},
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture, TextureFormat},
    ui::{FontFormat, UiCreator, UiFinder},
};

pub struct LoadingState {
    progress_counter: ProgressCounter,
    loading_gui: Option<Entity>,
    progress_bar: Option<Entity>,
}

impl LoadingState {
    pub fn new() -> LoadingState {
        LoadingState {
            progress_counter: ProgressCounter::new(),
            loading_gui: None,
            progress_bar: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        (*world.write_resource::<amethyst::controls::HideCursor>()).hide = false;

        self.loading_gui = Some(world.exec(
            |mut ui_creator: UiCreator<'_, AudioFormat, TextureFormat, FontFormat, CustomUi>| {
                ui_creator.create("ui/loading.ron", &mut self.progress_counter)
            },
        ));

        {
            let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
            let material = world.exec(|loader: AssetLoaderSystemData<Texture>| Material {
                albedo: loader
                    .load_from_data([0.0, 0.0, 1.0, 1.0].into(), &mut self.progress_counter),
                ..material_defaults
            });
            let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
                loader.load(
                    "mesh/suzanne.obj",
                    ObjFormat,
                    (),
                    &mut self.progress_counter,
                )
            });
            let transform = Transform::default();

            world
                .create_entity()
                .with(material)
                .with(mesh)
                .with(transform)
                .build();
        }

        graphics::initialize_light(world);
        graphics::initialize_camera(world);
    }

    fn update(&mut self, StateData { world, .. }: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Complete => {
                info!("Loading finished");

                world.delete_entity(self.loading_gui.unwrap()).unwrap();

                Trans::Push(Box::new(GameplayState::new()))
            }
            Completion::Loading => {
                /*
                // set loading gui entity, if it's been loaded
                if self.loading_gui.is_none() {
                    world.exec(|finder: UiFinder| {
                        if let Some(entity) = finder.find("loading_bar") {
                            self.loading_gui = Some(entity)
                        }
                    });
                }

                let mut custom_ui = world.write_storage::<???>();
                if let Some(loading_gui) = self.loading_gui.and_then(|e| custom_ui.get_mut(e)) {
                    // ...
                }
                */

                Trans::None
            }
            Completion::Failed => Trans::Quit,
        }
    }
}
