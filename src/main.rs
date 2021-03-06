#![allow(clippy::type_complexity)]

mod ecs;
mod graphics;
mod logger;
mod states;
mod ui;

use amethyst::{
    assets::Processor,
    audio::Source,
    controls::{CursorHideSystem, HideCursor, MouseFocusUpdateSystem},
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    ui::UiBundle,
    utils,
};
use clap::{App, Arg};
use ecs::CurrentState;
use logger::{prelude::*, Logger, UnwrapLog};
use states::LoadingState;
use std::path::PathBuf;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let clap = App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::with_name("color")
                .long("color")
                .short("c")
                .help("Enable console coloring"),
        )
        .get_matches();
    let color = clap.is_present("color");

    Logger::init(color, &["gfx_device_gl"]);

    let root_dir = PathBuf::from(utils::application_root_dir());
    let assets_path = root_dir.join("assets");
    let config_path = root_dir.join("config");

    let key_bindings_path = config_path.join("key_bindings.ron");
    let display_path = config_path.join("display.ron");

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(key_bindings_path)
        .unwrap_log("Failed to load key bindings");

    let game_data = Ok(GameDataBuilder::new())
        .and_then(|data| data.with_bundle(TransformBundle::new()))
        .and_then(|data| data.with_bundle(input_bundle))
        .map(|data| data.with(Processor::<Source>::new(), "source_processor", &[]))
        .map(|data| data.with(MouseFocusUpdateSystem::new(), "focus", &[]))
        .map(|data| data.with(CursorHideSystem::new(), "cursor_hide", &["focus"]))
        .map(|data| data.with(ecs::mainmenu::MainMenuRotation::new(0.7), "rotates", &[]))
        .map(|data| data.with(ecs::camera::CameraAspect::new(), "camera_aspect", &[]))
        .map(|data| {
            data.with(
                ecs::gameplay::CameraMovement::new(3.0),
                "camera_movement",
                &["focus"],
            )
        })
        .map(|data| {
            data.with(
                ecs::gameplay::CameraRotation::new(0.1, 0.1),
                "camera_rotation",
                &["focus"],
            )
        })
        .and_then(|data| data.with_bundle(UiBundle::<String, String>::new()))
        .and_then(|data| graphics::add_renderer(data, &display_path))
        .unwrap_log("Failed to create Game Data");

    let application = Application::build(assets_path, LoadingState::new())
        .map(|app| app.with_resource(HideCursor { hide: false }))
        .map(|app| app.with_resource(CurrentState::Loading))
        .and_then(|app| app.build(game_data));

    match application {
        Ok(mut game) => {
            info!("Starting {} [{}]...", NAME, VERSION);
            game.run();
        }
        Err(err) => {
            error!("Failed to initialize: {}", err);
        }
    }
}
