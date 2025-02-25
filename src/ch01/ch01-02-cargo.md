# cargo

## cargo是什么

cargo命令是rust语言官方提供的项目管理工具
通过cargo命令创建的项目依赖于Cargo.toml来配置项目和管理第三方依赖

## cargo怎么用

创建二进制程序项目

```sh
cargo new project_name
```

运行二进制程序项目

```sh
# debug模式
cargo run
# release
cargo run --release
```

创建库项目

```sh
cargo new --lib lib_name
```

运行库项目所有测试

```sh
cargo test
```
