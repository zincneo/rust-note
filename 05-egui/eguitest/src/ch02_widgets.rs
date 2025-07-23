// egui提供的组件widget
use eframe::egui;

pub(crate) fn run() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some([600.0, 1080.0].into()),
            titlebar_shown: Some(false),
            ..Default::default()
        },
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
    checked: bool,
    value: f32,
    slider_value: f32,
    radio_checked: bool,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            checked: true,
            value: 0.0,
            slider_value: 0.0,
            radio_checked: true,
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// egui提供的部分weight的简单使用
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 在中央面板上显示egui界面
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new("hello egui"));

            ui.add(egui::Button::new("Click me"));

            ui.add(egui::Checkbox::new(&mut self.checked, "Checked"));

            ui.add(egui::DragValue::new(&mut self.value).speed(0.1));

            ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=100.0).text("My value"));

            ui.add(egui::Hyperlink::new("https://github.com/emilk/egui"));

            let t = ui.add(egui::RadioButton::new(self.radio_checked, "Radio 1"));

            if t.clicked() {
                self.radio_checked = true;
            }

            let t = ui.add(egui::RadioButton::new(!self.radio_checked, "Radio 2"));

            if t.clicked() {
                self.radio_checked = false;
            }
        });
    }
}
