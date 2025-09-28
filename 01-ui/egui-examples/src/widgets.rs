use eframe::*;

#[derive(Default)]
pub struct Widgets {
    my_bool: bool,
    my_f32: f32,
    my_enum: Enum,
}

#[derive(PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

impl Default for Enum {
    fn default() -> Self {
        Self::First
    }
}

impl eframe::App for Widgets {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        use eframe::egui::widgets;
        egui::CentralPanel::default().show(ctx, |ui| {
            // 1. Button
            if ui.add(widgets::Button::new("Click me")).clicked() {
                println!("Button Clicked");
            }
            // 2. Checkbox
            ui.add(widgets::Checkbox::new(&mut self.my_bool, "Checked"));
            // 3. DragValue
            ui.add(
                widgets::DragValue::new(&mut self.my_f32)
                    .speed(0.2)
                    .range(0.0..=100.0),
            );
            // 4. Hyperlink
            ui.add(widgets::Hyperlink::new("https://bing.com"));
            let image = egui::include_image!("../assets/ferris.png");
            // 5. Image - need egui_extra
            ui.add(widgets::Image::new(image.clone()).max_width(200.));
            // 6. ImageButton
            if ui.add(egui::ImageButton::new(image)).clicked() {
                println!("ImageButton Clicked");
            };
            // 7. Label
            ui.add(widgets::Label::new("Label"));
            // 8. Link
            if ui.add(egui::Link::new("Documentation")).clicked() {
                println!("Link Clicked");
            }
            // 9. ProgressBar
            ui.add(widgets::ProgressBar::new(self.my_f32 / 100.).show_percentage());
            // 10. RadioButton
            [
                (Enum::First, "First"),
                (Enum::Second, "Second"),
                (Enum::Third, "Third"),
            ]
            .into_iter()
            .for_each(|e| {
                if ui
                    .add(egui::RadioButton::new(self.my_enum == e.0, e.1))
                    .clicked()
                {
                    self.my_enum = e.0;
                }
            });
            // 11. Separator
            ui.add(widgets::Separator::default());
            // 12. Slider
            ui.add(widgets::Slider::new(&mut self.my_f32, 0.0..=100.0).text("My value"));
            // 13. Spinner
            ui.add(widgets::Spinner::new());
        });
    }
}

impl Widgets {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

pub fn run() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 680.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "widgets",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Widgets::new(cc)))
        }),
    );
}
