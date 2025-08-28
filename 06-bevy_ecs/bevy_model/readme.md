# Bevy Model - 事件驱动的ECS系统

一个基于Bevy ECS构建的事件驱动模型系统，提供线程安全的UI状态管理和系统调度功能。

## 特性

- 🎯 **事件驱动架构** - 通过事件系统解耦业务逻辑
- 🔒 **线程安全** - 使用无锁数据结构和并发安全的设计
- 🎨 **UI状态管理** - 全局UI状态同步机制
- ⚡ **高性能** - 基于Bevy ECS的高性能实体组件系统
- 🧵 **多线程支持** - 独立的ECS处理线程

## 依赖

```toml
[dependencies]
bevy_ecs = "0.16.1"
crossbeam = { version = "0.8.4", features = ["crossbeam-channel"] }
arc-swap = "1.7.1"
dashmap = "6.1.0"
colored = "3.0.0"
log = "0.4.27"
env_logger = "0.11.8"
```

## 快速开始

### 基本使用

```rust
use bevy_model::{Model, event::BasicEvent, ui_store::UIInstanceId};

// 初始化系统
Model::init();

// 添加UI实例
let ui_id = Model::add_ui_instance(MyUIState::new());

// 推送系统到队列
Model::push_system(|schedule| {
    schedule.add_systems(my_system);
});

// 发送事件触发系统执行
Model::sender().send(Box::new(BasicEvent::Next)).unwrap();

// 获取UI状态
if let Some(ui_state) = Model::get_ui_instance::<MyUIState>(ui_id) {
    println!("UI State: {:?}", ui_state);
}

// 清理资源
Model::deinit();
```

### 自定义事件

```rust
use bevy_model::{Model, event::ModelEvent, ui_store::UIInstanceId};
use bevy_ecs::prelude::*;

#[derive(Debug)]
enum MyEvent {
    SpawnPlayer(UIInstanceId),
    UpdatePosition(f32, f32),
}

impl ModelEvent for MyEvent {
    fn handle(&self, world: &mut World) -> BasicEvent {
        match self {
            MyEvent::SpawnPlayer(ui_id) => {
                world.spawn(PlayerBundle::new(*ui_id));
            }
            MyEvent::UpdatePosition(x, y) => {
                // 更新位置逻辑
            }
        }
        BasicEvent::Nothing
    }
}

// 发送自定义事件
Model::sender().send(Box::new(MyEvent::SpawnPlayer(ui_id))).unwrap();
```

### UI状态管理

```rust
use bevy_model::{Model, ui_store::UIInstanceId};

#[derive(Clone, PartialEq, Debug)]
struct PlayerUI {
    health: f32,
    position: (f32, f32),
}

// 创建UI实例
let player_ui_id = Model::add_ui_instance(PlayerUI {
    health: 100.0,
    position: (0.0, 0.0),
});

// 更新UI状态
Model::set_instance(PlayerUI {
    health: 80.0,
    position: (10.0, 20.0),
}, player_ui_id);

// 获取UI状态
if let Some(ui_state) = Model::get_ui_instance::<PlayerUI>(player_ui_id) {
    println!("Player health: {}", ui_state.health);
}

// 移除UI实例
Model::remove_ui_instance(player_ui_id);
```

## 架构设计

### 核心组件

#### Model
全局模型管理器，提供以下功能：
- 事件发送通道管理
- ECS线程生命周期管理
- 系统队列管理
- UI状态存储管理

#### 事件系统
- `ModelEvent` trait：所有事件必须实现的接口
- `BasicEvent`：内置基础事件类型
- 事件处理器：在独立线程中处理事件并更新ECS世界

#### 系统队列
- 使用`SegQueue`实现的无锁系统队列
- 支持动态添加系统到调度器
- 在事件处理时执行系统

#### UI状态管理
- `UIStore`：线程安全的UI状态存储
- `UIInstanceId`：UI实例的唯一标识符
- 支持新旧状态比较和更新

### 线程模型

```
主线程/UI线程    事件通道    ECS线程
     |              |         |
     |-- 事件 -->   |         |
     |              |-- 事件 ->|
     |              |         |-- 处理事件
     |              |         |-- 执行系统
     |              |         |-- 更新世界
     |<-- UI状态 -- |         |
     |              |<-- 状态 -|
```

## API 参考

### Model

#### 生命周期管理

```rust
impl Model {
    /// 初始化系统，启动ECS处理线程
    pub fn init(&self);
    
    /// 清理系统，停止ECS处理线程
    pub fn deinit(&self) -> Option<()>;
}
```

#### 事件系统

```rust
impl Model {
    /// 获取事件发送器
    pub fn sender() -> Arc<Sender<Box<dyn ModelEvent>>>;
}
```

#### 系统管理

```rust
impl Model {
    /// 推送系统到执行队列
    pub fn push_system<T>(system: T)
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static;
}
```

#### UI状态管理

```rust
impl Model {
    /// 添加UI实例
    pub fn add_ui_instance<T>(instance: T) -> UIInstanceId
    where
        T: UITrait;
    
    /// 移除UI实例
    pub fn remove_ui_instance(instance_id: UIInstanceId);
    
    /// 获取UI实例状态
    pub fn get_ui_instance<T>(instance_id: UIInstanceId) -> Option<T>
    where
        T: UITrait;
    
    /// 设置UI实例状态
    pub fn set_instance<T>(instance: T, instance_id: UIInstanceId) -> Option<()>
    where
        T: UITrait;
}
```

### 事件系统

#### ModelEvent trait

```rust
pub trait ModelEvent: Debug + Sync + Send + 'static {
    /// 处理事件，返回基础事件
    fn handle(&self, world: &mut World) -> BasicEvent;
}
```

#### BasicEvent

```rust
pub enum BasicEvent {
    Stop(Stop),    // 停止ECS线程
    Next,          // 执行下一轮系统
    Nothing,       // 无操作
}
```

### UI状态管理

#### UIInstanceId

```rust
#[derive(PartialEq, Eq, Hash, Clone, Copy, Component, Debug)]
pub struct UIInstanceId(TypeId, u16);
```

#### UITrait

```rust
pub trait UITrait: Any + PartialEq + Clone + Send + Sync + 'static {}
```

## 最佳实践

### 1. 事件设计
- 事件应该是不可变的
- 一个事件只处理一个逻辑单元
- 使用枚举组织相关事件

### 2. 系统设计
- 系统应该是纯函数
- 避免在系统中直接访问UI状态
- 通过事件和组件进行通信

### 3. UI状态管理
- UI状态应该是可克隆的
- 实现`PartialEq`以支持状态比较
- 使用`UIInstanceId`进行状态关联

### 4. 错误处理
- 检查`Option`返回值
- 在关键路径添加错误处理
- 使用日志记录重要操作

## 示例项目

查看 `src/tests/cli_mock.rs` 了解完整的使用示例。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试并显示输出
just test
```

## 性能考虑

- 事件通道使用无界队列，注意内存使用
- UI状态更新会触发克隆，考虑使用引用计数
- 系统队列在每次事件处理时重建，避免频繁添加系统

## 限制和注意事项

1. **全局状态**：当前使用全局静态变量，难以进行单元测试
2. **错误处理**：部分API返回`Option`，缺少详细错误信息
3. **类型安全**：UI状态使用`Any`类型，失去编译时类型检查
4. **生命周期**：需要手动调用`init/deinit`，容易遗漏

## 贡献

欢迎提交Issue和Pull Request！

## 许可证

MIT License

