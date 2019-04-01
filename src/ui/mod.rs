mod loading_bar;

use amethyst::{
    audio::AudioFormat,
    renderer::TextureFormat,
    ui::{FontFormat, ToNativeWidget, UiTransformBuilder, UiWidget},
};
use loading_bar::LoadingBar;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum CustomUi {
    LoadingBar {
        transform: UiTransformBuilder,
        loading_bar: LoadingBar,
    },
}

impl ToNativeWidget for CustomUi {
    type PrefabData = ();

    fn to_native_widget(
        self,
        _parent_data: Self::PrefabData,
    ) -> (
        UiWidget<AudioFormat, TextureFormat, FontFormat, CustomUi>,
        Self::PrefabData,
    ) {
        match self {
            CustomUi::LoadingBar {
                loading_bar,
                transform,
            } => (loading_bar.to_native_widget(transform), ()),
        }
    }
}
