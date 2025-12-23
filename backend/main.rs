
use std::net::SocketAddr;
use axum::Server;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = routes::routes();
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	println!("Listening on http://{}", addr);
	Server::bind(&addr)
		.serve(app.into_make_service())
		.await?;
	Ok(())
}
