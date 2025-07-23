use eframe::egui::{self, FontId};

pub(crate) fn run() {
    // 创建视口选项，设置窗口大小
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    // 运行egui程序
    if let Err(e) = eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    ) {
        eprintln!("{:?}", e);
    }
}

struct MyEguiApp {
    count: i32,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 添加自定义的Widget
            let button_response = ui.add(CustomButton {
                label: self.count.to_string(),
                size: (120.0, 80.0).into(),
                background_color: egui::Color32::from_rgb(255, 134, 0),
                foreground_color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 255),
            });
            if button_response.clicked() {
                self.count += 1;
            }
        });
    }
}

struct CustomButton {
    label: String,
    size: egui::Vec2,
    background_color: egui::Color32,
    foreground_color: egui::Color32,
}

impl egui::Widget for CustomButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = FontId::proportional(16.0);
        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect_filled(rect, egui::CornerRadius::same(8), self.background_color);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                self.label,
                font_id,
                self.foreground_color,
            );
        }
        response
    }
}
