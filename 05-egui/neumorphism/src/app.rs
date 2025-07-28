use eframe::egui::{self, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp;

impl MyApp {
    const BASE_COLOR: Color32 = Color32::from_rgb(224, 224, 224);
    const TEXT_COLOR: Color32 = Color32::from_rgb(0, 31, 63);
    const TEXT_OPACITY_COLOR: Color32 =
        Color32::from_rgba_premultiplied(0, 31, 63, (255. * 0.8) as u8);
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
                    ui.add_space(35.);
                    component::header(ctx, ui);
                    ui.add_space(50.);
                    component::button_preview(ctx, ui);
                });
            });
    }
}

mod widget {

    use serde::{Deserialize, Serialize};

    use super::*;

    pub struct Header;

    impl Widget for Header {
        fn ui(self, ui: &mut Ui) -> Response {
            let font_id = FontId::proportional(35.0);
            let max_rect = ui.max_rect();
            let response = ui.allocate_rect(max_rect, Sense::empty());
            ui.painter().text(
                max_rect.center(),
                Align2::CENTER_CENTER,
                "Neumorphism.io".to_string(),
                font_id,
                MyApp::TEXT_COLOR,
            );
            let font_id = FontId::proportional(16.0);
            ui.painter().text(
                max_rect.center_bottom(),
                Align2::CENTER_BOTTOM,
                "Generate neumorphic designs".to_string(),
                font_id,
                MyApp::TEXT_OPACITY_COLOR,
            );
            response
        }
    }

    #[derive(Clone, Copy)]
    pub enum Direction {
        RightTop,
        LeftTop,
        LeftDown,
        RightDown,
    }

    impl Into<i32> for Direction {
        fn into(self) -> i32 {
            match self {
                Self::RightTop => 0,
                Self::LeftTop => 1,
                Self::LeftDown => 2,
                Self::RightDown => 3,
            }
        }
    }

    impl Into<Direction> for i32 {
        fn into(self) -> Direction {
            match self {
                0 => Direction::RightTop,
                1 => Direction::LeftTop,
                2 => Direction::LeftDown,
                _ => Direction::RightDown,
            }
        }
    }

    impl Direction {
        fn get_angle(&self) -> (f32, f32) {
            use widget::Direction::*;
            match self {
                RightDown => (0.0, std::f32::consts::FRAC_PI_2),
                LeftDown => (std::f32::consts::FRAC_PI_2, std::f32::consts::PI),
                LeftTop => (std::f32::consts::PI, std::f32::consts::FRAC_PI_2 * 3.0),
                RightTop => (std::f32::consts::FRAC_PI_2 * 3.0, std::f32::consts::TAU),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeumorphismInfo {
        size: i32,
        radius: i32,
        distance: i32,
        insentity: i32,
        blur: i32,
        is_down: bool,
    }

    pub fn neumorphism_container(
        ui: &mut Ui,
        center: Pos2,
        info: NeumorphismInfo,
        add_contents: impl FnOnce(&mut Ui),
    ) {
        let rect_size = Vec2::new(info.size as f32, info.size as f32);
        let rect = Rect::from_center_size(center, rect_size);

        let _ = ui.allocate_rect(rect, Sense::empty());

        let base_offset = Vec2::new(info.distance as f32, info.distance as f32);
        let base_rect = rect.translate(base_offset);
        let reverse_offset = Vec2::new(-info.distance as f32, -info.distance as f32);
        let reverse_base_rect = rect.translate(reverse_offset);
        let blur_layers = info.distance;
        let dark_color;
        {
            let start_color = 0xde;
            let end_color = 0x5a;
            dark_color = start_color - (start_color - end_color) / 10 * info.insentity;
        }

        for i in 0..blur_layers {
            let offset = Vec2::new(i as f32, i as f32);
            let reverse_offset = Vec2::new(-i as f32, -i as f32);
            let alpha = 255 - (255 / info.distance * i);
            let shadow_rect = base_rect.translate(offset);
            let reverse_shadow_rect = reverse_base_rect.translate(reverse_offset);
            let dark_color = Color32::from_rgba_unmultiplied(
                dark_color as u8,
                dark_color as u8,
                dark_color as u8,
                alpha as u8,
            );
            ui.painter().rect(
                shadow_rect,
                CornerRadius::same(info.radius as u8),
                if i == 0 {
                    dark_color
                } else {
                    Color32::TRANSPARENT
                },
                Stroke::new(2., dark_color),
                StrokeKind::Inside,
            );
            ui.painter().rect(
                reverse_shadow_rect,
                CornerRadius::same(info.radius as u8),
                dark_color,
                Stroke::NONE,
                StrokeKind::Inside,
            );
        }
        ui.painter().rect(
            rect,
            CornerRadius::same(info.radius as u8),
            MyApp::BASE_COLOR,
            Stroke::NONE,
            StrokeKind::Inside,
        );
        ui.scope_builder(UiBuilder::default().max_rect(rect), |child_ui| {
            add_contents(child_ui);
        });
    }

    pub struct LeftPreview;

    impl LeftPreview {
        const CIRCLE_RADIUS: f32 = 30.;
        fn draw_quater_circle(ui: &mut Ui, center: Pos2, direction: widget::Direction, fill: bool) {
            let painter = ui.painter();
            let steps = 32;
            let mut points = vec![center];
            let (start_angle, end_angle) = direction.get_angle();
            for i in 0..=steps {
                let angle = start_angle + (end_angle - start_angle) * (i as f32 / steps as f32);
                let x = center.x + Self::CIRCLE_RADIUS * angle.cos();
                let y = center.y + Self::CIRCLE_RADIUS * angle.sin();
                points.push(Pos2::new(x, y));
            }
            let stroke = Stroke::new(2., MyApp::TEXT_OPACITY_COLOR);
            let fill = if fill {
                Color32::YELLOW
            } else {
                Color32::TRANSPARENT
            };
            painter.add(Shape::convex_polygon(points, fill, stroke));
        }
        pub fn add_quarter_circle(center: Pos2, direction: widget::Direction, ui: &mut Ui) {
            use widget::Direction::*;
            let direction_checked_id = ui.make_persistent_id("direction_id");
            let mut direction_checked = ui
                .ctx()
                .data_mut(|d| d.get_persisted::<i32>(direction_checked_id))
                .unwrap_or(widget::Direction::LeftDown.into());
            let fill = direction_checked == direction.into();
            let start = match direction {
                RightDown => Pos2::new(center.x, center.y),
                RightTop => Pos2::new(center.x, center.y - Self::CIRCLE_RADIUS),
                LeftTop => Pos2::new(
                    center.x - Self::CIRCLE_RADIUS,
                    center.y - Self::CIRCLE_RADIUS,
                ),
                LeftDown => Pos2::new(center.x - Self::CIRCLE_RADIUS, center.y),
            };
            let rect =
                Rect::from_min_size(start, [Self::CIRCLE_RADIUS, Self::CIRCLE_RADIUS].into());
            let response = ui.allocate_rect(rect, Sense::click());
            if response.clicked() {
                direction_checked = direction.into();
                ui.ctx()
                    .data_mut(|d| d.insert_persisted(direction_checked_id, direction_checked));
            }
            Self::draw_quater_circle(ui, center, direction, fill);
        }
    }
    impl Widget for LeftPreview {
        fn ui(self, ui: &mut Ui) -> Response {
            let direction_checked_id = ui.make_persistent_id("direction_id");
            ui.ctx()
                .data_mut(|d| d.get_persisted::<i32>(direction_checked_id))
                .unwrap_or(widget::Direction::RightDown.into());
            let max_rect = ui.max_rect();
            let response = ui.allocate_response(max_rect.size(), Sense::empty());
            let (left_top, left_bottom, right_top, right_bottom) = (
                max_rect.left_top(),
                max_rect.left_bottom(),
                max_rect.right_top(),
                max_rect.right_bottom(),
            );
            use widget::Direction::*;
            [
                (left_top, RightDown),
                (left_bottom, RightTop),
                (right_top, LeftDown),
                (right_bottom, LeftTop),
            ]
            .into_iter()
            .for_each(|arg| LeftPreview::add_quarter_circle(arg.0, arg.1, ui));

            ui.horizontal_centered(|ui| {
                neumorphism_container(
                    ui,
                    max_rect.center(),
                    NeumorphismInfo {
                        size: 200,
                        radius: 20,
                        distance: 5,
                        insentity: 10,
                        blur: 5,
                        is_down: true,
                    },
                    |_| {},
                );
            });

            response
        }
    }

    pub struct RightSetting;
}

mod component {
    use super::*;

    pub fn header(ctx: &egui::Context, ui: &mut Ui) {
        let screen_rect = ctx.screen_rect();
        let screen_width = screen_rect.width();
        let screen_height = screen_rect.height();
        let top_height = (0.1 * screen_height).max(50.0);
        ui.add_sized([screen_width, top_height], widget::Header);
    }

    pub fn button_preview(ctx: &egui::Context, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            let width = ui.available_width();
            let space_width: f32 = 10.;
            let component_width = ((width - space_width * 3.) / 2.).min(500.);
            const COMPONENT_HEIGHT: f32 = 500.;
            let space_width = (width - (component_width * 2.0)) / 3.0;
            web_sys::console::log_1(&space_width.into());
            ui.add_space(space_width);
            ui.add_sized([component_width, COMPONENT_HEIGHT], widget::LeftPreview);
            ui.add_space(space_width);
            ui.add_sized([component_width, COMPONENT_HEIGHT], Button::new("right"));
            ui.add_space(space_width);
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
