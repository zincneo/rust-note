use eframe::*;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl eframe::App for Counter {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // 1. ctx的方法style_mut修改的是全局的样式配置
        ctx.style_mut(|style| {
            style.spacing.item_spacing = (0.0, 0.0).into();
        });
        // 2. 对于Panel的样式配置通过frame方法
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                fill: egui::Color32::from_rgb(255, 255, 255),
                stroke: egui::Stroke::NONE,
                corner_radius: egui::CornerRadius::ZERO,
                outer_margin: egui::Margin::ZERO,
                shadow: egui::Shadow::NONE,
            })
            .show(ctx, |ui| {
                ui.heading("Counter");
                ui.horizontal(|ui| {
                    // 3. 通过scope产生一个局部作用域，这个局部的style修改只对闭包内有效
                    // style_mut/visuals_mut方法
                    ui.scope(|ui| {
                        ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                        if ui
                            .add(
                                egui::widgets::Button::new("Decrement")
                                    .min_size((100., 30.).into())
                                    .corner_radius(egui::CornerRadius::same(0)),
                            )
                            .clicked()
                        {
                            self.value -= 1;
                        }
                    });
                    ui.scope(|ui| {
                        let style = ui.style_mut();
                        style.visuals.override_text_color = Some(egui::Color32::GREEN);
                        let label = egui::widgets::Label::new(format!("{}", self.value));
                        ui.add(label);
                    });

                    ui.scope(|ui| {
                        let visuals = ui.visuals_mut();
                        visuals.override_text_color = Some(egui::Color32::BLUE);
                        visuals.widgets.hovered.weak_bg_fill = egui::Color32::RED;
                        if ui
                            .add(
                                egui::widgets::Button::new("Increment")
                                    .min_size((100., 30.).into())
                                    .corner_radius(egui::CornerRadius::same(0)),
                            )
                            .clicked()
                        {
                            self.value += 1;
                        }
                    });
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
