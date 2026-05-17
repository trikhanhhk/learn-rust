use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = routes::create_router();
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("Server running on: http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
