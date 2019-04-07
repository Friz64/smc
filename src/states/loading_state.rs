use crate::{
    ecs::CurrentState,
    graphics,
    logger::prelude::*,
    states::{GameplayData, MainMenuData, MainMenuState},
    ui::{self, CustomUi},
};
use amethyst::{
    assets::{AssetLoaderSystemData, Completion, Handle, ProgressCounter},
    audio::{output, AudioFormat},
    controls::HideCursor,
    core::{specs::Entity, transform::components::ParentHierarchy, Transform},
    prelude::*,
    renderer::{Material, MaterialDefaults, Mesh, ObjFormat, Texture, TextureFormat},
    ui::{FontFormat, UiCreator, UiFinder, UiLoader, UiPrefab, UiText, UiTransform},
};

pub struct LoadingState {
    progress_counter: ProgressCounter,
    gameplay_data: Option<GameplayData>,
    mainmenu_data: Option<MainMenuData>,
    mainmenu_gui: Option<Handle<UiPrefab>>,
    loading_gui: Option<Entity>,
    progress_bar: Option<Entity>,
}

impl LoadingState {
    pub fn new() -> LoadingState {
        LoadingState {
            progress_counter: ProgressCounter::new(),
            gameplay_data: None,
            mainmenu_data: None,
            mainmenu_gui: None,
            loading_gui: None,
            progress_bar: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, StateData { world, .. }: StateData<GameData>) {
        *world.write_resource::<CurrentState>() = CurrentState::Loading;

        output::init_output(&mut world.res);

        self.mainmenu_gui = Some(world.exec(
            |ui_loader: UiLoader<'_, AudioFormat, TextureFormat, FontFormat, CustomUi>| {
                ui_loader.load("ui/mainmenu.ron", &mut self.progress_counter)
            },
        ));

        self.loading_gui = Some(world.exec(
            |mut ui_creator: UiCreator<'_, AudioFormat, TextureFormat, FontFormat, CustomUi>| {
                ui_creator.create("ui/loading.ron", &mut self.progress_counter)
            },
        ));

        self.gameplay_data = Some(GameplayData::load(world, &mut self.progress_counter));
        self.mainmenu_data = Some(MainMenuData::load(world, &mut self.progress_counter));
    }

    fn on_stop(&mut self, StateData { world, .. }: StateData<GameData>) {
        world.delete_entity(self.loading_gui.unwrap()).unwrap();
    }

    fn update(&mut self, StateData { world, .. }: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Complete => {
                info!("Loading finished");

                Trans::Switch(Box::new(MainMenuState::new(
                    self.mainmenu_gui.as_ref().unwrap().clone(),
                    self.mainmenu_data.as_ref().unwrap().clone(),
                    self.gameplay_data.as_ref().unwrap().clone(),
                )))
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
