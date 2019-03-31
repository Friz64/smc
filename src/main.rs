mod graphics;
//mod input;
mod logger;
mod states;
mod systems;

use amethyst::{
    controls::{CursorHideSystem, MouseFocusUpdateSystem},
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    ui::UiBundle,
    utils,
};
use clap::{App, Arg};
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
    let resources_path = root_dir.join("resources");

    let key_bindings_path = resources_path.join("key_bindings.ron");
    let display_path = resources_path.join("display.ron");

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(key_bindings_path)
        .unwrap_log("Failed to load key bindings");

    let game_data = Ok(GameDataBuilder::new())
        .and_then(|data| data.with_bundle(TransformBundle::new()))
        .and_then(|data| data.with_bundle(input_bundle))
        .map(|data| data.with(MouseFocusUpdateSystem::new(), "focus", &[]))
        .map(|data| data.with(CursorHideSystem::new(), "cursor_hide", &["focus"]))
        .map(|data| data.with(systems::CameraAspect::new(), "camera_aspect", &[]))
        .map(|data| {
            data.with(
                systems::CameraMovement::new(3.0),
                "camera_movement",
                &["focus"],
            )
        })
        .map(|data| {
            data.with(
                systems::CameraRotation::new(0.1, 0.1),
                "camera_rotation",
                &["focus"],
            )
        })
        .map(|data| data.with(systems::UiEventHandler::new(), "ui_event_handler", &[]))
        .and_then(|data| data.with_bundle(UiBundle::<String, String>::new()))
        .and_then(|data| graphics::add_renderer(data, &display_path))
        .unwrap_log("Failed to create Game Data");

    match Application::new(assets_path, LoadingState::new(), game_data) {
        Ok(mut game) => {
            info!("Starting {} [{}]...", NAME, VERSION);
            game.run();
        }
        Err(err) => {
            error!("Failed to initialize: {}", err);
        }
    }
}
