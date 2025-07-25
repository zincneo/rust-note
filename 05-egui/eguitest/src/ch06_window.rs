use eframe::egui::{self, *};

// 创建窗口的一些配置参数
pub(crate) fn run() {
    // 创建视口选项，设置窗口大小
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            decorations: Some(false), // 无边框窗口
            transparent: Some(true),  // 透明窗口
            ..Default::default()
        }
        .with_always_on_top(), // 至于顶层
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
        // 创建透明窗口
        CentralPanel::default()
            .frame(Frame::NONE.fill(Color32::from_rgba_unmultiplied(0, 0, 0, 0)))
            .show(ctx, |_| {
                // 通过ctx拿到键盘输入ESC时退出
                if ctx.input(|input| input.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
    }
}
