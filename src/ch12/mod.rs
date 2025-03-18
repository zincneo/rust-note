/// ## 文档行注释
/// - rust提供了`cargo doc`命令可以将文档注释转换为HTML网页
/// - 使用///表示文档行注释
/// - 文档注释必须位于lib类型的crate当中
/// - 文档注释可以使用markdown语法
/// - 文档注释的对象必须是pub对外可见的
fn _ch12_01_doc_comment() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch12_01() {
        assert_eq!(_ch12_01_doc_comment(), ());
    }
}
