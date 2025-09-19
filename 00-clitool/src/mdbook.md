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

### SUMMARY

```md
# Summary
[Introduction](README.md)
- [My First Chapter](my-first-chapter.md)
- [Nested example](nested/README.md)
    - [Sub-chapter](nested/sub-chapter.md)
```

1. 标题
    ```md
    # Summary
    ```
2. 预览章节 不在markdown列表下的顶层连接，在目录的开头
    ```md
    [A Prefix Chapter](relative/path/to/markdown.md)
    ```
3. 子标题
    ```md
    # My Part Title
    ```
4. 序列化章节
    ```md
    # Title of Part

    - [First Chapter](relative/path/to/markdown.md)
    - [Second Chapter](relative/path/to/markdown2.md)
        - [Sub Chapter](relative/path/to/markdown3.md)

    # Title of Another Part

    - [Another Chapter](relative/path/to/markdown4.md)
    ```
5. 后缀章节 和预览章节一样不在markdown列表下，但在目录的末尾
    ```md
    [Title of Suffix Chapter](relative/path/to/markdown2.md)
    ```
6. 弃用的章节 在url中不配置地址
    ```md
    - [Draft Chapter]()
    ```
7. 分隔符
    ```md
    # My Part Title

    [A Prefix Chapter](relative/path/to/markdown.md)

    ---

    - [First Chapter](relative/path/to/markdown2.md)
    ```

完整的一个目录配置示例

```md
# Summary

[Introduction](README.md)

# User Guide

- [Installation](guide/installation.md)
- [Reading Books](guide/reading.md)
- [Creating a Book](guide/creating.md)

# Reference Guide

- [Command Line Tool](cli/README.md)
    - [init](cli/init.md)
    - [build](cli/build.md)
    - [watch](cli/watch.md)
    - [serve](cli/serve.md)
    - [test](cli/test.md)
    - [clean](cli/clean.md)
    - [completions](cli/completions.md)
- [Format](format/README.md)
    - [SUMMARY.md](format/summary.md)
        - [Draft chapter]()
    - [Configuration](format/configuration/README.md)
        - [General](format/configuration/general.md)
        - [Preprocessors](format/configuration/preprocessors.md)
        - [Renderers](format/configuration/renderers.md)
        - [Environment Variables](format/configuration/environment-variables.md)
    - [Theme](format/theme/README.md)
        - [index.hbs](format/theme/index-hbs.md)
        - [Syntax highlighting](format/theme/syntax-highlighting.md)
        - [Editor](format/theme/editor.md)
    - [MathJax Support](format/mathjax.md)
    - [mdBook-specific features](format/mdbook.md)
    - [Markdown](format/markdown.md)
- [Continuous Integration](continuous-integration.md)
- [For Developers](for_developers/README.md)
    - [Preprocessors](for_developers/preprocessors.md)
    - [Alternative Backends](for_developers/backends.md)

-----------

[Contributors](misc/contributors.md)
```
