use leptos::prelude::*;

/**
# 组件和属性
- 通过component宏声明且返回值为impl IntoView的方法可以作为view!中的组件使用
- 组件的属性在参数列表中传入，子组件接收父组件中的状态可以使用ReadSignal类型来接收
- view!宏中使用自定义组件传入参数的方式`<ComponentName propName=value />`
- 在自定义组件的参数列表中可以通过宏指属性是否可选以及默认值
    - `#[prop(optional)]`
    - `#[prop(default = value)]`
- 参数类型也可以指定为闭包`impl Fn() -> T + Send + Sync + 'static`
*/
#[component]
pub fn Element01_component() -> impl IntoView {
    let (count, set_count) = signal(0);
    let button_click = move |_| *set_count.write() += 5;
    let double_count = move || count.get() * 2;
    #[component]
    fn ProgressBar(
        #[prop(optional)]
        #[prop(default = 100)]
        max: u16,
        progress: ReadSignal<i32>,
    ) -> impl IntoView {
        view! { <progress max=max value=progress /> }
    }
    #[component]
    fn ProgressBar2(progress: impl Fn() -> i32 + Send + Sync + 'static) -> impl IntoView {
        view! { <progress max=100 value=progress /> }
    }
    view! {
        <button on:click=button_click>"Click me"</button>
        <ProgressBar max=50 progress=count />
        <ProgressBar2 progress=double_count />
    }
}
