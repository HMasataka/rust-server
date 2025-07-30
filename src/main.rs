use axum::{routing::get, serve, Router};
use rust_server::usecase::dto::calculation_dto::HelloResponse;
use rust_server::presentation::handlers::hello_handler;
use tokio::net::TcpListener;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(rust_server::presentation::handlers::hello_handler::hello_handler),
    components(schemas(HelloResponse)),
    tags(
        (name = "Hello World", description = "Onion Architecture API")
    )
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    let app = Router::new().route("/", get(hello_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
