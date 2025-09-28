use eframe::egui;
pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "layout",
        native_options,
        Box::new(|cc| Ok(Box::new(Layout::new(cc)))),
    );
}

#[derive(Default)]
struct Layout {}

impl eframe::App for Layout {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // egui内部提供整体布局，窗口提供顶部底部区域、侧边区域、中间区域
        // 中间区域默认存在，如果其他部分不禁止绘制则中间区域占满整个窗口
        // egui::TopBottomPanel::top()
        // egui::SidePanel::left()
        // egui::SidePanel::right()
        egui::CentralPanel::default().show(ctx, |ui| {
            // 1. 默认ui布局每个widgets从上到下
            ui.label("label - 1");
            ui.label("label - 2");
            // 2. 使用方法ui.with_layout/allocate_ui_with_layout方法 传入指定布局方式的结构体egui::Layout
            // 简化方法horizontal/vertical
            let remain_rect = ui.available_rect_before_wrap(); // 剩余空间
            let mut rect = egui::Rect::ZERO;
            rect.min = remain_rect.min;
            (rect.max.x, rect.max.y) = (rect.min.x + 200., rect.min.y + 200.);
            let painter = ui.painter();
            painter.rect_stroke(
                rect,
                egui::CornerRadius::same(0),
                egui::Stroke::new(1., egui::Color32::from_rgb(255, 0, 0)),
                egui::StrokeKind::Outside,
            );
            let layout = egui::Layout {
                main_dir: egui::Direction::RightToLeft, // ui元素的主轴流动方向
                main_wrap: true, // 主方向上是否自动换行 换行空间不够的情况下会再按照desired_size分配一个
                main_align: egui::Align::Center, // 主方向上的对齐方式
                main_justify: false, // 控制主轴方向是否尽量获得最大宽度/高度
                cross_align: egui::Align::Center, // 垂直主轴方向的对齐方式
                cross_justify: true, // 垂直主轴方向是否获得最大值(水平布局部件获得最大高度，垂直布局部件获得最大宽度)
            };
            ui.allocate_ui_with_layout((200., 200.).into(), layout, |ui| {
                ui.label("label - 3");
                ui.label("label - 4");
                ui.label("label - 5");
            });

            // 3. grid布局 egui::Grid
            // 内部使用ui.end_row来结束一行，每行内可以放置任意控件
            egui::Grid::new("my_grid") // 每一个grid需要唯一id
                .striped(false)
                .show(ui, |ui| {
                    ui.label("name: ");
                    ui.text_edit_singleline(&mut String::new());
                    ui.end_row();

                    ui.label("age: ");
                    ui.text_edit_singleline(&mut String::new());
                    ui.end_row();
                });
        });
        // egui::TopBottomPanel::bottom()
    }
}

impl Layout {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
