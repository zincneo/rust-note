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

### 处理流程

- 全局变量中包含的字段一开始均为空或者包裹None
- init方法会初始化必须的环境，并启动一个独立的线程专门处理ecs，其他线程通过全局变量上的Sender发送事件与ecs进行交互
- deinit方法清理当前的Sender并发送停止事件然后阻塞当前线程直到ecs线程结束
