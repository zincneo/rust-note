use eframe::{
    CreationContext,
    egui::{self, Widget},
};

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "custom counter",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

// 在组件上存Id类型，ui上使用id来获取到Context保存的对应值
struct Counter {
    id: egui::Id,
}

impl Counter {
    pub fn new(id_source: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id_source),
        }
    }
}

impl Widget for Counter {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let inner_response = ui.horizontal(|ui| {
            let id = self.id;
            // ui.data 实则会调用到Context类型上的data
            let mut value = ui.data(|data| data.get_temp::<i32>(id).unwrap_or(0));
            if ui.button("-").clicked() {
                value -= 1;
            }
            ui.label(format!("{}", value));
            if ui.button("+").clicked() {
                value += 1;
            }
            ui.data_mut(|data| data.insert_temp(id, value));
        });
        inner_response.response
    }
}

#[derive(Default)]
struct App {}

impl App {
    fn new(_: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Counter::new("first"));
            ui.add(Counter::new("first"));
            ui.add(Counter::new("second"));
        });
    }
}
