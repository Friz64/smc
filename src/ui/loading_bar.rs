use super::CustomUi;
use amethyst::{
    assets::AssetPrefab,
    audio::AudioFormat,
    renderer::{TextureFormat, TexturePrefab},
    ui::{Anchor, FontAsset, FontFormat, UiImageBuilder, UiTransformBuilder, UiWidget},
};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct LoadingBar {
    pub background_texture: TexturePrefab<TextureFormat>,
    pub bar_texture: TexturePrefab<TextureFormat>,
    pub progress: f32,
    pub font: Option<AssetPrefab<FontAsset, FontFormat>>,
    pub font_size: f32,
}

impl LoadingBar {
    pub fn to_native_widget(
        self,
        background_transform: UiTransformBuilder,
    ) -> UiWidget<AudioFormat, TextureFormat, FontFormat, CustomUi> {
        let loading_bar_transform = {
            let spacing = 10.0;
            let width = (background_transform.width - spacing) * self.progress;
            UiTransformBuilder {
                x: (width / 2.0) + (background_transform.x + (spacing / 2.0)),
                width,
                height: background_transform.height - spacing,
                anchor: Anchor::MiddleLeft,
                ..Default::default()
            }
        };

        let loading_bar = UiWidget::Image {
            transform: loading_bar_transform,
            image: UiImageBuilder {
                image: self.bar_texture,
            },
        };

        UiWidget::Container {
            background: Some(UiImageBuilder {
                image: self.background_texture,
            }),
            transform: background_transform,
            children: vec![loading_bar],
        }
    }
}
