use args::ARGS;

mod args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    kostats_web::host(ARGS.port, ARGS.db.clone(), ARGS.redis.clone()).await;
    Ok(())
}
