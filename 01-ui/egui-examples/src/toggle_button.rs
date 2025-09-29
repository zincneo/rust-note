use eframe::egui;
pub fn run() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "toggle-button",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

// 立即模式下做动画也和保留模式不同，立即模式每一帧都重新绘制，所以其实就是计算每一帧应该绘制到什么位置
pub struct ToggleButton<'a> {
    value: &'a mut bool,
    size: egui::Vec2,
}

impl<'a> ToggleButton<'a> {
    pub fn new(value: &'a mut bool) -> Self {
        Self {
            value,
            size: egui::vec2(40.0, 20.0), // 默认大小
        }
    }

    pub fn with_size(mut self, size: egui::Vec2) -> Self {
        self.size = size;
        self
    }
}

impl<'a> egui::Widget for ToggleButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = ui.make_persistent_id(self.value as *const _ as usize);

        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::click());

        if response.clicked() {
            *self.value = !*self.value;
        }

        // 动画进度：0.0 ~ 1.0
        let raw_t = ui.ctx().animate_bool(id, *self.value);

        // 自定义缓动（ease in-out cubic）
        let t = if raw_t < 0.5 {
            4.0 * raw_t * raw_t * raw_t
        } else {
            1.0 - (-2.0 * raw_t + 2.0).powi(3) / 2.0
        };

        let painter = ui.painter();
        let radius = rect.height() / 2.0;

        // 背景条
        let bg_color = if *self.value {
            egui::Color32::from_rgb(100, 200, 100)
        } else {
            egui::Color32::from_gray(120)
        };
        let border_color = egui::Color32::from_rgba_premultiplied(66, 66, 66, (0.6 * 255.) as u8);

        painter.rect_filled(rect, radius, bg_color);
        painter.rect_stroke(
            rect,
            radius,
            egui::Stroke::new(2., border_color),
            egui::StrokeKind::Outside,
        );

        // 滑块位置
        let circle_x = egui::lerp(rect.left() + radius..=rect.right() - radius, t);
        let center = egui::pos2(circle_x, rect.center().y);
        painter.circle_filled(center, radius * 0.8, egui::Color32::WHITE);

        response
    }
}

pub struct App {
    toggled: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { toggled: false }
    }
}

impl App {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(ToggleButton::new(&mut self.toggled).with_size(egui::vec2(80.0, 30.0)));
            ui.label(format!("toggled:{}", self.toggled));
        });
    }
}
