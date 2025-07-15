use super::*;
#[derive(Parser, Debug)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get/post
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其它 HTTP 方法
}

pub fn parse_url(s: &str) -> Result<String> {
    // 这里我们仅仅检查一下 URL 是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}

// get 子命令
/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
pub struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    pub url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
pub struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    pub url: String,

    /// HTTP 请求的 body (key=value 格式)
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

pub fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}
