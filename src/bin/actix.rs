use std::env;
mod actix_mod;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let Some(arg0) = args.get(1) else {
        return Ok(());
    };
    match arg0.as_str() {
        "hello-world" => actix_mod::hello::main().await,
        _ => Ok(()),
    }
}
