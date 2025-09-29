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

// 由于框架的设计哲学差异，这里和web前端的诸如Rect这样的框架有较大的差异
// web前端是保留模式，因此组件内部可以保存状态，下一次渲染继续使用内部的状态
// egui则是即时模式，也就是当前组件下一帧是新建新的对象，就的已经依照rust所有权早就消耗掉了，因此我们无法在组件内部存状态
// 第一种解决方式是通过外部传引用，然后把状态放在App上
// 第二种解决方法是通过egui::Context(在context示例中介绍)
struct Counter<'a> {
    value: &'a mut i32,
}

impl<'a> Counter<'a> {
    pub fn new(value: &'a mut i32) -> Self {
        Self { value }
    }
}

impl<'a> Widget for Counter<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let inner_response = ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                *self.value -= 1;
            }
            ui.label(format!("{}", *self.value));
            if ui.button("+").clicked() {
                *self.value += 1;
            }
        });
        inner_response.response
    }
}

#[derive(Default)]
struct App {
    value: i32,
}

impl App {
    fn new(_: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Counter::new(&mut self.value));
            ui.add(Counter::new(&mut self.value));
        });
    }
}
