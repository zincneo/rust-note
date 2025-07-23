// egui 是一个有布局和交互功能的 2D 用户界面库。 egui 无法知道它的运行环境，也不知道如何获取输入/输出到显示器。 这是 集成 或 后端 的任务。
// 在游戏引擎中使用 egui 是很常见的（比如 bevy_egui）， 但你也可以依靠 eframe 来单独使用 egui。eframe 有着 Web 和 Native，处理输入和渲染的集成。 eframe 中的 frame 既代表 egui app 中的 帧（frame），又代表框架（framework）（eframe 是个框架, egui 是个库）。
use eframe::egui;

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
