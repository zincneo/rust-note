use eframe::egui;

pub(crate) fn run() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 600.0]),
        ..Default::default()
    };
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui下提供的直接在顶层布局UI的API
        // TopBottomPannel::top()/bottom()
        // SidePannel::left()/right()
        // CentralPanel::default()
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui本身垂直排列，也就是使用ui.add添加组件将会从上到下排布
            ui.heading("hello");
            // 1. 水平布局/垂直布局
            // horizontal创建水平布局(从上到下) verical创建垂直布局(从左到右)
            ui.horizontal(|ui| {
                // 现在在该闭包中会水平排布添加的组件
                ui.label("first line");
                if ui.button("second button").clicked() {
                    println!("click second button");
                }
            });
            // 离开了horizontal的闭包回到原来的布局模式
            ui.heading("hi");
            // 嵌套水平和垂直布局
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.button("button1-1").clicked() {}
                    if ui.button("button2-1").clicked() {}
                });
                ui.vertical(|ui| {
                    if ui.button("button1-2").clicked() {}
                    if ui.button("button2-2").clicked() {}
                })
            });
            // 2. Grid布局 适合做表单
            // 通过Grid::new创建，ui作为show的参数，然后在闭包中操作，end_row表示一行结束
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();

                // 这里可以看出一行的高度会对齐本行最高的UI
                ui.vertical(|ui| {
                    ui.label("Same");
                    ui.label("cell");
                });
                ui.label("Third row, second column");
                ui.end_row();
            });
            // 填充、对齐、间距控制
            // 3. 填充/间距add_space
            ui.add_space(5.0);
            // 4. 准确指定布局和对齐方 with_layout
            // 5. 像前端一样实现在空间内均匀排布 like css display: flex; justify-content: space-between/space-around;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                const BUTTON_WIDTH: f32 = 30.0;
                let spacing = (ui.available_width() - BUTTON_WIDTH * 3.0) / 4.0; // 假设3个按钮 → 分成4段
                ui.add_space(spacing / 2.0); // 左边边距
                // 指定按钮的宽度
                ui.add_sized([BUTTON_WIDTH, 0.0], egui::Button::new("button-1"));
                ui.add_space(spacing);
                ui.add_sized([BUTTON_WIDTH, 0.0], egui::Button::new("button-2"));
                ui.add_space(spacing);
                ui.add_sized([BUTTON_WIDTH, 0.0], egui::Button::new("button-3"));
                ui.add_space(spacing / 2.0); // 右边边距
            });
        });
    }
}
