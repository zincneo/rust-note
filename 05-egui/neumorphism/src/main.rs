#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    use neumorphism::MyApp;

    // 将 log 的信息重定向到 js 的 console.log
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions {
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        // 在 id 为 canvas 的元素上创建 egui 的画面
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Failed to find canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        // 启动
        let _ = eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|_| Ok(Box::new(MyApp))))
            .await;
    });
}
