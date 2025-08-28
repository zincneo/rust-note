# 基于bevy_ecs实现的模型结构

## 依赖的库

- bevy_ecs 提供核心的ecs系统
- corssbeam 提供多线程消息通道
- arc-swap 提供读取无锁的智能指针
- dashmap 提供具有并发读取能力的哈希表

## 架构设计

> 核心结构体Model，提供全局变量MODEL

### Model字段解析

- sender 存储通道的发送端，可以被安全的Clone到其他线程中
- handle ecs处理线程的句柄，在清理的时候用于阻塞到ecs线程结束
- system_queue 全局无锁队列，用于存储下一次执行计划表时要执行的system
- ui_store 支持并发的哈希表，提供ui状态的读写

### 处理流程

- 全局变量中包含的字段一开始均为空或者包裹None
- init方法会初始化必须的环境，并启动一个独立的线程专门处理ecs，其他线程通过全局变量上的Sender发送事件与ecs进行交互
- deinit方法清理当前的Sender并发送停止事件然后阻塞当前线程直到ecs线程结束
- sender关联方法，获取发送端
- push_system关联方法，添加下一轮要执行的system
- add_ui_instance关联方法
- get_ui_instance关联方法
  - 该方法会先通过读方式(无锁)访问哈希表，如果值不是None的话才会再按照写方式读哈希表然后将值拿走留下一个None在原来的位置
  - 这里的设计是为了读的时候频繁上锁，且当能够读到Some的时候才会去更新ui
- remove_ui_instance关联方法

