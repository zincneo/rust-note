use eframe::egui::{self, *};

pub(crate) fn run() {
    // 创建视口选项，设置窗口大小
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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

impl eframe::App for MyEguiApp {
    // ctx是egui 的 GUI 上下文对象，提供对输入、状态、屏幕绘制、窗口生命周期等所有东西的访问
    // 可以用来获取输入input()/存储状态data()/禁用交互set_enable(bool)/设置获取样式set_style()/style()
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Counter::new("counter"));
            // 监听输入
            ctx.input(|input| {
                if input.key_pressed(egui::Key::Enter) {
                    println!("Enter key pressed");
                }
            });
        });
    }
}

struct Counter {
    id: String,
}

impl Counter {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl Widget for Counter {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(self.id);
        // 获取跨帧的长期存储的值
        let mut count = ui
            .ctx()
            .data_mut(|d| d.get_persisted::<i32>(id))
            .unwrap_or(0);

        let response = ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                count -= 1;
                ui.ctx().data_mut(|d| d.insert_persisted(id, count));
            }
            ui.label(format!("Count: {}", count));
            if ui.button("+").clicked() {
                count += 1;
                ui.ctx().data_mut(|d| d.insert_persisted(id, count));
            }
        });
        response.response
    }
}
