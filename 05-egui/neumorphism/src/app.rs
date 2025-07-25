use eframe::egui::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp;

impl MyApp {
    const BASE_COLOR: Color32 = Color32::from_rgb(224, 224, 224);
    const TEXT_COLOR: Color32 = Color32::from_rgb(0, 31, 63);
    const TEXT_OPACITY_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 31, 63, 160);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        ctx.layer_painter(LayerId::background())
            .rect_filled(screen_rect, 0.0, MyApp::BASE_COLOR);
        set_style(ctx);
        Window::new("neumorphism")
            .fixed_rect(screen_rect)
            .title_bar(false)
            .resizable(false)
            .frame(Frame::NONE.fill(MyApp::BASE_COLOR))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    component::header(ctx, ui);
                    component::button_preview(ctx, ui);
                });
            });
    }
}

mod component {
    use super::*;
    pub fn header(ctx: &egui::Context, ui: &mut Ui) {
        let screen_rect = ctx.screen_rect();
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        let top_height = (0.1 * screen_height).max(50.0);
        let top_rect = Rect::from_min_size(screen_rect.min, [screen_width, top_height].into());
        let font_id = FontId::proportional(35.0);
        let painter = ui.painter();
        painter.rect_filled(top_rect, CornerRadius::ZERO, MyApp::BASE_COLOR);
        painter.text(
            top_rect.center(),
            Align2::CENTER_CENTER,
            "Neumorphism.io".to_string(),
            font_id,
            MyApp::TEXT_COLOR,
        );
        let font_id = FontId::proportional(21.0);
        painter.text(
            top_rect.center_bottom(),
            Align2::CENTER_BOTTOM,
            "Generate neumorphic designs".to_string(),
            font_id,
            MyApp::TEXT_OPACITY_COLOR,
        );
    }
    pub fn button_preview(ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let width = ui.available_width();
            let rect_width = (width - 30.) / 2.;
            ui.add_space(10.);
            let (rect, _) = ui.allocate_exact_size([rect_width, 400.0].into(), Sense::empty());
            let font_id = FontId::proportional(40.0);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "Left".to_string(),
                font_id,
                MyApp::TEXT_OPACITY_COLOR,
            );
            ui.painter().rect(
                rect,
                8.0,
                Color32::TRANSPARENT,
                Stroke {
                    width: 1.,
                    color: MyApp::TEXT_COLOR,
                },
                StrokeKind::Inside,
            );
            ui.add_space(10.);
            let (rect, _) = ui.allocate_exact_size([rect_width, 400.0].into(), Sense::empty());
            let font_id = FontId::proportional(40.0);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "right".to_string(),
                font_id,
                MyApp::TEXT_OPACITY_COLOR,
            );
            ui.painter().rect(
                rect,
                8.0,
                Color32::TRANSPARENT,
                Stroke {
                    width: 1.,
                    color: MyApp::TEXT_COLOR,
                },
                StrokeKind::Inside,
            );
        });
    }
}

fn set_style(ctx: &egui::Context) {
    ctx.set_style({
        let mut style = (*ctx.style()).clone();
        let widgets = &mut style.visuals.widgets;
        widgets.inactive.corner_radius = CornerRadius::ZERO;
        widgets.hovered.corner_radius = CornerRadius::ZERO;
        widgets.active.corner_radius = CornerRadius::ZERO;
        widgets.open.corner_radius = CornerRadius::ZERO;
        widgets.inactive.bg_fill = MyApp::BASE_COLOR;
        widgets.hovered.bg_fill = MyApp::BASE_COLOR;
        widgets.active.bg_fill = MyApp::BASE_COLOR;
        widgets.open.bg_fill = MyApp::BASE_COLOR;
        style.visuals.extreme_bg_color = MyApp::BASE_COLOR;
        style.visuals.window_fill = MyApp::BASE_COLOR;
        style.visuals.panel_fill = MyApp::BASE_COLOR;

        style
    });
}
