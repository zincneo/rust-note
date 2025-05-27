use leptos::prelude::*;
mod ch01;
mod ch02;
fn main() {
    mount_to_body(App);
}

/**
# 挂载函数
- leptos设计上与React非常相似
- view! {}中的接收的语法与jsx非常相似
- 与jsx不同的是直接支持多个标签，不需要像react中一样提供<></>包裹
*/
#[component]
fn App() -> impl IntoView {
    view! {
        <ch01::Element01_state />
        <ch01::Element02_event />
        <ch01::ELement03_attribute />
        <ch02::Element01_component />
        <ch02::Element02_iteration />
        <ch02::Element03_dynamic />
    }
}
