use amethyst::{
    core::{
        nalgebra::{Perspective3, Quaternion, Translation3, UnitQuaternion, Vector3},
        specs::prelude::*,
        Transform,
    },
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawShaded, Light, Pipeline, PointLight, PosNormTex, RenderBundle,
        Rgba, Stage,
    },
    ui::DrawUi,
};

use std::path::PathBuf;

const INIT_WIDTH: u32 = 1280;
const INIT_HEIGHT: u32 = 720;

pub fn initialize_camera(world: &mut World) {
    let transform = Transform::new(
        Translation3::new(0.0, 0.0, -4.0),
        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 1.0, 0.0)),
        Vector3::new(1.0, 1.0, 1.0),
    );

    let camera = Camera {
        proj: Perspective3::new(
            INIT_WIDTH as f32 / INIT_HEIGHT as f32,
            45.0f32.to_radians(),
            0.1,
            2000.0,
        )
        .to_homogeneous(),
    };

    world.create_entity().with(transform).with(camera).build();
}

pub fn initialize_light(world: &mut World) {
    // light
    let light: Light = PointLight {
        intensity: 3.0,
        color: Rgba::white(),
        radius: 5.0,
        ..Default::default()
    }
    .into();

    let transform = Transform::new(
        Translation3::new(2.0, 2.0, -2.0),
        UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 1.0, 0.0)),
        Vector3::new(1.0, 1.0, 1.0),
    );

    world.create_entity().with(light).with(transform).build();
}

pub fn add_renderer<'a, 'b>(
    game_data_builder: GameDataBuilder<'a, 'b>,
    resources_path: &PathBuf,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    let mut display_config = DisplayConfig::load(resources_path);
    display_config.title = crate::NAME.into();
    display_config.dimensions = Some((INIT_WIDTH, INIT_HEIGHT));

    let pipeline = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.3, 0.3, 0.3, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    game_data_builder.with_bundle(RenderBundle::new(pipeline, Some(display_config)))
}
