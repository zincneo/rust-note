use eframe::egui::{self, CornerRadius, Stroke, StrokeKind};

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "painter",
        native_options,
        Box::new(|cc| Ok(Box::new(Painter::new(cc)))),
    );
}

#[derive(Default)]
struct Painter;

impl Painter {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Painter {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let _ = egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Painter类型用来在当前frame中绘制内容
            // painter() 获取到不可变引用
            // allocate_painter() -> (Response, Painter)
            // 绘制内容的方法
            // 1. painter自身提供的方法
            {
                let desired_size = (300., 200.).into();
                let (response, mut painter) =
                    ui.allocate_painter(desired_size, egui::Sense::click());
                if response.clicked() {
                    println!("Painter-1 Clicked");
                }
                let mut rect = painter.clip_rect();
                rect.min.x += 100.;
                painter.set_opacity(0.5);
                painter.rect_filled(
                    rect,
                    egui::CornerRadius::same(0),
                    egui::Color32::from_rgba_premultiplied(255, 0, 0, 128),
                );
                let mut center = rect.center();
                center.x -= 50.;
                let radius = rect.height() / 2.0;
                painter.circle_filled(
                    center,
                    radius,
                    egui::Color32::from_rgba_premultiplied(0, 255, 0, 128),
                );
            }
            // 2. 通过Mesh类型来精确控制如何绘制
            {
                use egui::epaint::{Color32, Mesh};
                let mut mesh = Mesh::default();
                let desired_size = (300., 200.).into();
                let (response, mut painter) =
                    ui.allocate_painter(desired_size, egui::Sense::click());
                if response.clicked() {
                    println!("Painter-2 Clicked");
                }
                let rect = painter.clip_rect();
                let mut rect = rect;
                rect.min.x += 100.;
                painter.set_opacity(0.5);
                add_rect(
                    &mut mesh,
                    rect,
                    Color32::from_rgba_premultiplied(0, 0, 255, 128),
                );
                let rect = painter.clip_rect();
                let mut rect = rect;
                rect.max.x -= 100.;
                add_rect(
                    &mut mesh,
                    rect,
                    Color32::from_rgba_premultiplied(0, 255, 0, 128),
                );
                painter.add(egui::Shape::mesh(mesh));
            }
            // 3. Shape类型提供一些基本图形API
            {
                use eframe::epaint::{CircleShape, RectShape, Shape};
                let desired_size = (300., 200.).into();
                let (response, mut painter) =
                    ui.allocate_painter(desired_size, egui::Sense::click());
                if response.clicked() {
                    println!("Painter-3 Clicked");
                }
                painter.set_opacity(0.5);
                let mut rect = painter.clip_rect();
                rect.min.x += 150.;
                let mut shapes = vec![];
                let rect_shape = RectShape::new(
                    rect,
                    CornerRadius::same(0),
                    Color32::from_rgba_premultiplied(255, 0, 0, 128),
                    Stroke::NONE,
                    StrokeKind::Inside,
                );
                let rect = painter.clip_rect();
                let center = rect.center();
                let radius = rect.height() / 2.0;
                shapes.push(Shape::Rect(rect_shape));
                let circle_shape = CircleShape::filled(
                    center,
                    radius,
                    Color32::from_rgba_premultiplied(0, 255, 0, 128),
                );
                shapes.push(Shape::Circle(circle_shape));
                painter.extend(shapes);
            }
        });
    }
}

use eframe::egui::{
    Pos2, Rect,
    epaint::{Color32, Mesh, Vertex},
};
fn add_rect(mesh: &mut Mesh, rect: Rect, color: Color32) {
    let a = rect.left_top();
    let b = rect.right_top();
    let c = rect.right_bottom();
    let d = rect.left_bottom();
    mesh.vertices.push(Vertex {
        pos: a,
        uv: Pos2::ZERO,
        color,
    });

    let start = mesh.vertices.len() as u32;
    [a, b, c, d]
        .into_iter()
        .map(|e| Vertex {
            pos: e,
            uv: Pos2::ZERO,
            color,
        })
        .for_each(|e| {
            mesh.vertices.push(e);
        });

    mesh.indices
        .extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
}
