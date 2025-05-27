use leptos::prelude::*;

/**
# 控制流
- 在view!宏中可以{}执行Rust闭包函数
- 因此可以在{}的闭包中使用Rust的流程控制语句
*/
#[component]
pub fn Element01_control_flow() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;
    let message = move || {
        if is_odd() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };
    view! {
        <button on:click=move |_| set_value.update(|n| { *n += 1 })>"+1"</button>
        // if else
        <p>{move || if is_odd() { "Odd" } else { "Even" }}</p>
        // Option
        <p>{message}</p>
        // match
        <p>
            {move || {
                match value.get() {
                    0 => "Zero",
                    1 => "One",
                    _n if is_odd() => "Odd",
                    _ => "Even",
                }
            }}
        </p>
    }
}
