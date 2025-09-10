use tokio::net::TcpListener;
use tokio::signal;
use dummy_redis_server::server;

#[tokio::main]
async fn main() -> dummy_redis_server::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    server::run(listener,signal::ctrl_c()).await;
    
    Ok(())
}
