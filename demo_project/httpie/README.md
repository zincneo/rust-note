# httpie

## 需求

- httpie本身是一个替代curl和wget的命令行http客户端程序
- 通过rust实现一个简单的httpie
  1. 实现命令行解析，处理子命令和各种参数，验证用户的输入，并且将这些输入转换成程序能理解的参数
  2. 根据解析好的参数，发送一个 HTTP 请求，获得响应
  3. 用对用户友好的方式输出响应

## 用到的crates

- anyhow 错误处理
- clap 命令行解析
- colored 命令行多彩输出
- jsonxf 格式化
- mime 处理mime类型
- reqwest http客户端
- tokio 异步运行时库
