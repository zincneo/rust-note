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

/**
# 标签属性
- class
    1. 通过class:classname直接指定
    2. 在view!中不直接支持的字符使用class="classname"包裹指定
    3. 动态指定某个或者多个类名
        1. class:classname=闭包->bool
        2. class=("classname", 闭包->bool)
        3. class=(["classname", ...], 闭包->bool)
- style
    1. 通过style:stylename=value直接指定
    2. 通过style="stylename=value"指定
    3. 动态style属性值
        1. style:stylename=闭包->&str
        2. style=("stylename", 闭包->&str)
- 其他属性
    1. attribute:ReadSignal<T>
    2. attribute=闭包->值
*/
#[component]
pub fn ELement03_attribute() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button
            value=count
            on:click=move |_| *set_count.write() += 1
            style:position="absolute"
            style:top="0px"
            style:left=move || format!("{}px", count.get() * 10 + 100)
            class=(["red", "button-20", "rounded"], move || count.get() % 2 == 1)
        >
            {count}
        </button>
    }
}
