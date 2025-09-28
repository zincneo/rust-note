use eframe::egui::{self, RichText, widgets};
pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([400.0, 100.0])
            .with_min_inner_size([400.0, 100.0])
            .with_transparent(true), // To have rounded corners we need transparency
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Custom window frame", // unused title
        options,
        Box::new(|cc| Ok(Box::new(Frame::new(cc)))),
    );
}

#[derive(Default)]
struct Frame {}

impl Frame {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Frame {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let button_height = 12.0;
                let close_response = ui
                    .add(widgets::Button::new(
                        RichText::new("❌").size(button_height),
                    ))
                    .on_hover_text("Close the window");
                if close_response.clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
    }
}
