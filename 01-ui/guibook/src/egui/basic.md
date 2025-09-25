# egui

[项目地址](https://github.com/emilk/egui)

- egui本身只是图形库，并非完整框架，需要结合完整的图形框架使用(eframe/bevy)
- immediate-model GUI 立即模式 每一帧都重新绘制而非保留原有状态，因此没有回调

## 计数器

```rs
use eframe::*;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl eframe::App for Counter {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
            ui.horizontal(|ui| {
                if ui.button("Decrement").clicked() {
                    self.value -= 1;
                }
                ui.spacing();
                ui.label(format!("{}", self.value));
                if ui.button("Increment").clicked() {
                    self.value += 1;
                }
            });
        });
    }
}

impl Counter {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
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
```
