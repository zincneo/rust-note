// egui纯Rust编写的即时模式图形用户界面(GUI库)
// eframe配合egui使用的框架，提供与平台无关的窗口和事件处理能力
use eframe::egui;

pub(crate) struct Hello;

impl Hello {
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
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// 实现eframe::App特征
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 在中央面板上显示egui界面
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello egui");
        });
    }
}
