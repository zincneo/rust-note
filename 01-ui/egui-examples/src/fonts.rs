use eframe::egui;
use std::sync::Arc;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl eframe::App for Counter {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("计数器");
            ui.horizontal(|ui| {
                if ui.button("减少").clicked() {
                    self.value -= 1;
                }
                ui.spacing();
                ui.label(format!("{}", self.value));
                if ui.button("增加").clicked() {
                    self.value += 1;
                }
            });
        });
    }
}

impl Counter {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "counter",
        native_options,
        Box::new(|cc| Ok(Box::new(Counter::new(cc)))),
    );
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let font_data = Arc::new(egui::FontData::from_static(include_bytes!(
        "../assets/MapleMono-NF-CN-Regular.ttf"
    )));
    fonts.font_data.insert("maple".to_owned(), font_data);

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "maple".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("maple".to_owned());

    ctx.set_fonts(fonts);
}
