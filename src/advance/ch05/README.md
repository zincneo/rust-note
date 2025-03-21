# Concurrent and Parallel Programming

## 并发和并行的概念

以排队买咖啡举例:
- 并发(Concurrent)
  - 只有一个售卖口，排两条队伍轮换处理
  - 计算机同一个时刻处理一个任务
  - 切换处理的速度够快对于用户观感上就有多个任务同时在运行
- 并行(Parallel)
  - 有多个售卖口，同时处理多个队伍
  - 计算机同一个时刻处理多个任务

## CPU核心数的影响

> 操作系统处理任务的最小单位是线程，一个程序可能存在一个或多个进程，一个进程又可以有一个或者多个线程

### 单核

没法并行，只能并发

### 多核

并行和并发通常是混合的

## 编程语言的并发模型

- 由于操作系统提供了创建线程的 API，因此部分语言会直接调用该 API 来创建线程，因此最终程序内的线程数和该程序占用的操作系统线程数相等，一般称之为1:1 线程模型，例如 Rust、C这些
- 还有些语言在内部实现了自己的线程模型（绿色线程、协程），程序内部的 M 个线程最后会以某种映射方式使用 N 个操作系统线程去运行，因此称之为M:N 线程模型，其中 M 和 N 并没有特定的彼此限制关系。一个典型的代表就是 Go 语言
- 还有些语言使用了 Actor 模型，基于消息传递进行并发，例如 Erlang 语言
