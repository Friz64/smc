use crate::{graphics, logger::prelude::*, states::GameplayState};
use amethyst::{
    assets::{AssetLoaderSystemData, Completion, ProgressCounter},
    core::{Transform, specs::Entity},
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture},
    ui::UiCreator,
};

pub struct LoadingState {
    progress_counter: ProgressCounter,
    loading_text: Option<Entity>,
}

impl LoadingState {
    pub fn new() -> LoadingState {
        LoadingState {
            progress_counter: ProgressCounter::new(),
            loading_text: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        self.loading_text = Some(world.exec(|mut ui_creator: UiCreator| {
            ui_creator.create("ui/loading.ron", &mut self.progress_counter)
        }));

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

                world.delete_entity(self.loading_text.unwrap()).unwrap();

                Trans::Push(Box::new(GameplayState::new()))
            }
            Completion::Loading => Trans::None,
            Completion::Failed => Trans::Quit,
        }
    }
}
