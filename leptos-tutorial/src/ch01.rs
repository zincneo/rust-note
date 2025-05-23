use leptos::prelude::*;

/**
# 状态控制
- leptops提供了signal方法用来实现组件的状态控制
- signal方法返回一个元组(ReadSignal<T>, WriteSignal<T>)
- view!{} 内部支持{}语法将rust变量转换为UI上的值显示
    1. 传入ReadSignal<T>类型将会得响应式的变化，内存中T类型的值发生改变，UI也会发生改变
    2. 传入T类型的值则只会渲染一次，如果通过WriteSignal<T>修改包裹的值不会更新UI上的值
    3. 另外一种同步UI变化的写法是通过闭包函数返回T类型的值，内存中通过WriteSignal修改包裹的值闭包会重新执行
*/
#[component]
pub fn Element01_state() -> impl IntoView {
    let (count, _) = signal(0);
    view! {
        // 1. 响应式
        <p>{count}</p>
        // 2. 非响应式
        <p>{count.get()}</p>
        // 3. 通过闭包实现响应式
        <p>{move || count.get()}</p>
    }
}

/**
# 监听事件
- 在html标签上直接使用on::event_name=闭包
*/
#[component]
pub fn Element02_event() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <p on:click=move |_| set_count.set(3)>"click event"{count}</p> }
}
