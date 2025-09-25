# Rust Note

- 00 clitool: cargo工具链提供的命令行工具
  - mdbook - 通过markdown构建书籍的命令行工具，方便部署到web端
- 01 ui: Rust制作gui程序的一些框架对比
  - guibook - 框架对比
  - egui-examples - egui的一些例子
- 02 thread: Rust线程相关的用法/库
  - native - 标准库线程相关的用法
  - crossbeam - 提供一些并发工具
    - crossbeam_channel 性能比官方库更好，而且是多发多收通道，并且提供超时API和强大的select宏
  - parking_lot - 互斥锁、读写锁的替代
