mod config;
mod extension;
mod input;
mod plugin;

pub use config::EguiConfig;
pub use egui::{load::SizedTexture, *};
pub use extension::{EguiCallbackFn, EguiExtension, EguiRegisterTexture};
pub use plugin::{EguiPlugin, EguiPluginSugar};

pub mod extras {
    pub use egui_extras::*;
}
