# mdbook

[详细的官方手册](https://rust-lang.github.io/mdBook/)

## 安装

```sh
cargo install mdbook
```

安装git上的最新版本

```sh
cargo install --git https://github.com/rust-lang/mdBook.git mdbook
```

## 创建项目

```sh
mdbook init project-name
```

## 启动项目

```sh
mdbook serve --open
```

## 项目文件解析

```sh
project-name/
├── book/ 项目生成文件目录
├── book.toml 项目配置文件
└── src/
    ├── chapter_1.md
    └── SUMMARY.md 包含整个项目的章节定义(左侧索引树)
```
