mod app;
pub use app::*;
mod widget;
use eframe::egui::*;
pub use widget::*;

pub const BASE_COLOR: Color32 = Color32::from_rgb(224, 224, 224);
pub const TEXT_COLOR: Color32 = Color32::from_rgb(0, 31, 63);
pub const TEXT_OPACITY_COLOR: Color32 =
    Color32::from_rgba_premultiplied(0, 31, 63, (255. * 0.8) as u8);
