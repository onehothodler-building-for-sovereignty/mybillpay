use std::net::SocketAddr;
use axum::Server;
use mybillpay_backend::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
