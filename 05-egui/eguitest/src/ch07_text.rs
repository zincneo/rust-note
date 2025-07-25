use eframe::{
    egui::{self, *},
    emath::Rot2,
    epaint::TextShape,
};

pub(crate) fn run() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    ) {
        eprintln!("{:?}", e);
    }
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                Text::default()
                    .with_content("test")
                    .with_fontsize(20.0)
                    .with_angle(0.0)
                    .with_forcecolor(Color32::RED),
            );
            ui.add(
                Text::default()
                    .with_content("test")
                    .with_fontsize(20.0)
                    .with_angle(2.0 * 3.14159 / 90.0)
                    .with_forcecolor(Color32::RED),
            );
            ui.add(
                Text::default()
                    .with_content("test")
                    .with_fontsize(20.0)
                    .with_angle(1.2)
                    .with_forcecolor(Color32::RED),
            );
        });
    }
}

struct Text {
    content: Option<String>,
    fontsize: Option<f32>,
    forcecolor: Option<Color32>,
    angle: Option<f32>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: None,
            fontsize: None,
            forcecolor: None,
            angle: None,
        }
    }
}

impl Text {
    fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    fn with_fontsize(mut self, size: f32) -> Self {
        self.fontsize = Some(size);
        self
    }
    fn with_forcecolor(mut self, color: impl Into<Color32>) -> Self {
        self.forcecolor = Some(color.into());
        self
    }
    fn with_angle(mut self, angle: f32) -> Self {
        self.angle = Some(angle);
        self
    }
}

impl Widget for Text {
    fn ui(self, ui: &mut Ui) -> Response {
        let color = self.forcecolor.unwrap_or(Color32::BLACK);
        let font_id = FontId::proportional(self.fontsize.unwrap_or(8.0));
        let Some(ref text) = self.content else {
            return ui.allocate_rect(Rect::ZERO, Sense::empty());
        };
        let text = text.clone();
        let angle = self.angle.unwrap_or(0.0);
        let galley = ui.painter().layout_no_wrap(text, font_id, color);
        let rotation = Rot2::from_angle(angle);
        let (rect, response) = {
            let bounding_rect =
                Rect::from_center_size(Pos2::ZERO, galley.size()).rotate_bb(rotation);
            ui.allocate_exact_size(bounding_rect.size(), Sense::empty())
        };
        if ui.is_rect_visible(rect) {
            let pos = rect.center() - (rotation * (galley.size() / 2.0));

            ui.painter().add(TextShape {
                angle: self.angle.unwrap_or(0.0),
                ..TextShape::new(pos, galley, color)
            });
        }
        response
    }
}
