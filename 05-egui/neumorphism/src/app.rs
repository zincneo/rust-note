use eframe::egui::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        ctx.layer_painter(LayerId::background())
            .rect_filled(screen_rect, 0.0, crate::BASE_COLOR);
        set_style(ctx);
        Window::new("neumorphism")
            .fixed_rect(screen_rect)
            .title_bar(false)
            .resizable(false)
            .frame(Frame::NONE.fill(crate::BASE_COLOR))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(35.);
                    ui.add(crate::widget::Header);
                    ui.add_space(50.);
                    ui.add(crate::widget::ButtonPreview);
                });
            });
    }
}

fn set_style(ctx: &egui::Context) {
    ctx.set_style({
        use crate::BASE_COLOR;
        let mut style = (*ctx.style()).clone();
        let widgets = &mut style.visuals.widgets;
        widgets.inactive.corner_radius = CornerRadius::ZERO;
        widgets.hovered.corner_radius = CornerRadius::ZERO;
        widgets.active.corner_radius = CornerRadius::ZERO;
        widgets.open.corner_radius = CornerRadius::ZERO;
        widgets.inactive.bg_fill = BASE_COLOR;
        widgets.hovered.bg_fill = BASE_COLOR;
        widgets.active.bg_fill = BASE_COLOR;
        widgets.open.bg_fill = BASE_COLOR;
        style.visuals.extreme_bg_color = BASE_COLOR;
        style.visuals.window_fill = BASE_COLOR;
        style.visuals.panel_fill = BASE_COLOR;
        style
    });
}
