use super::CustomUi;
use amethyst::{
    assets::AssetPrefab,
    audio::AudioFormat,
    renderer::{TextureFormat, TexturePrefab},
    ui::{
        Anchor, FontAsset, FontFormat, UiImageBuilder, UiText, UiTextBuilder, UiTransform,
        UiTransformBuilder, UiWidget,
    },
};
use serde::Deserialize;

const SPACING: f32 = 10.0;

#[derive(Clone, Deserialize)]
pub struct UiLoadingBar {
    pub background_texture: TexturePrefab<TextureFormat>,
    pub bar_texture: TexturePrefab<TextureFormat>,
    pub progress: f32,
    pub font: Option<AssetPrefab<FontAsset, FontFormat>>,
    pub font_color: [f32; 4],
    pub font_size: f32,
}

impl UiLoadingBar {
    pub fn native_widget(
        self,
        background_transform: UiTransformBuilder,
    ) -> UiWidget<AudioFormat, TextureFormat, FontFormat, CustomUi> {
        let loading_bar = {
            let loading_bar_transform = {
                let width = (background_transform.width - SPACING) * self.progress;
                UiTransformBuilder {
                    x: (width / 2.0) + (background_transform.x + (SPACING / 2.0)),
                    width,
                    height: background_transform.height - SPACING,
                    anchor: Anchor::MiddleLeft,
                    ..Default::default()
                }
            };

            UiWidget::Image {
                transform: loading_bar_transform,
                image: UiImageBuilder {
                    image: self.bar_texture,
                },
            }
        };

        let loading_text = {
            let loading_text_transform = UiTransformBuilder {
                z: 2.0,
                width: background_transform.width,
                height: background_transform.height,
                anchor: Anchor::Middle,
                ..Default::default()
            };

            UiWidget::Text {
                transform: loading_text_transform,
                text: UiTextBuilder {
                    text: "0.00%".into(),
                    font_size: self.font_size,
                    color: self.font_color,
                    font: self.font,
                    password: false,
                    align: Some(Anchor::Middle),
                    line_mode: None,
                    editable: None,
                },
            }
        };

        UiWidget::Container {
            background: Some(UiImageBuilder {
                image: self.background_texture,
            }),
            transform: background_transform,
            children: vec![loading_bar, loading_text],
        }
    }
}

pub fn update_loading_bar(
    bar_transform: &mut UiTransform,
    background_transform: UiTransform,
    loading_text: &mut UiText,
    progress: f32,
) {
    let width = (background_transform.width - SPACING) * progress;
    bar_transform.local_x = (width / 2.0) + (background_transform.local_x + (SPACING / 2.0));
    bar_transform.width = width;

    loading_text.text = format!("{}%", (progress * 100.0).round());
}
