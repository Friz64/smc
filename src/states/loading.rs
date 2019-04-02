use crate::{
    graphics,
    logger::prelude::*,
    states::GameplayState,
    ui::{self, CustomUi},
};
use amethyst::{
    assets::{AssetLoaderSystemData, Completion, ProgressCounter},
    audio::AudioFormat,
    core::{specs::Entity, transform::components::ParentHierarchy, Transform},
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture, TextureFormat},
    ui::{FontFormat, UiCreator, UiFinder, UiText, UiTransform},
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
        self.loading_gui = Some(world.exec(
            |mut ui_creator: UiCreator<'_, AudioFormat, TextureFormat, FontFormat, CustomUi>| {
                ui_creator.create("ui/loading.ron", &mut self.progress_counter)
            },
        ));

        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
        let material = world.exec(|loader: AssetLoaderSystemData<Texture>| Material {
            albedo: loader.load_from_data([0.0, 0.0, 1.0, 1.0].into(), &mut self.progress_counter),
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

        // fake loading test
        {
            for i in 0..100 {
                use amethyst::assets::{Progress, Tracker};
                (&mut self.progress_counter).add_assets(1);
                let tracker = Box::new(self.progress_counter.create_tracker());
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(i * 20));
                    tracker.success();
                });
            }
        }

        world
            .create_entity()
            .with(material)
            .with(mesh)
            .with(transform)
            .build();

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
                if let Some(progress_bar) = self.progress_bar {
                    let progress = self.progress_counter.num_finished() as f32
                        / self.progress_counter.num_assets() as f32;

                    let hierarchy = world.read_resource::<ParentHierarchy>();
                    let children = hierarchy.children(progress_bar);

                    let mut transforms = world.write_storage::<UiTransform>();
                    let background_transform = transforms.get(progress_bar).cloned();
                    let bar_transform = children.get(0).and_then(|&e| transforms.get_mut(e));

                    let mut texts = world.write_storage::<UiText>();
                    let loading_text = children.get(1).and_then(|&e| texts.get_mut(e));

                    if let (Some(background_transform), Some(bar_transform), Some(loading_text)) =
                        (background_transform, bar_transform, loading_text)
                    {
                        ui::update_loading_bar(
                            bar_transform,
                            background_transform,
                            loading_text,
                            progress,
                        );
                    }
                } else {
                    world.exec(|finder: UiFinder| {
                        if let Some(entity) = finder.find("progress_bar") {
                            self.progress_bar = Some(entity)
                        }
                    });
                }

                Trans::None
            }
            Completion::Failed => Trans::Quit,
        }
    }
}
