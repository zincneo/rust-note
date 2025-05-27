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

/**
# 静态迭代
- 在{}中可以通过Vec<_>获取静态视图
- Vec内容只会渲染一次，之后内存中Vec新增或者删除元素UI上不会动态更新
*/
#[component]
pub fn Element02_iteration() -> impl IntoView {
    let values = [0, 1, 2, 3, 4];
    // 创建多个signal
    let counters = (1..=5).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                </li>
            }
        })
        .collect_view();
    view! {
        <p>{values.clone()}</p>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
        <ul>{counter_buttons}</ul>
    }
}

/**
# 动态迭代
- Leptos提供`For`组件生成的动态视图
    1. each属性值为一个方法或者闭包，返回值必须实现IntoIter特征
    2. key属性值为一个方法或者闭包，返回值用作每个迭代生成时Dom元素对应的标识，用来计算何时替换DOM元素
    3. childern属性值为一个方法或者闭包，返回值用作每个迭代生成view
*/
#[component]
pub fn Element03_dynamic() -> impl IntoView {
    let initial_length = 0_usize;
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };
    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button on:click=move |_| *count.write() += 1>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .write()
                                        .retain(|(counter_id, _)| { counter_id != &id });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
