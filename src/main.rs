use axum::{routing::get, serve, Json, Router};
use rust_server::math::calculator;
use rust_server::utils::helper;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(hello_handler),
    components(schemas(Hello)),
    tags(
        (name = "Hello World", description = "A simple hello world API")
    )
)]
pub struct ApiDoc;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Hello {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "Hello World",
    responses(
        (status = 200, description = "Hello World", body = Hello),
    ),
)]
pub async fn hello_handler() -> Json<Hello> {
    println!("=== 別ディレクトリの関数を呼び出すサンプル ===\n");

    // mathディレクトリのcalculatorモジュールの関数を使用
    let a = 10;
    let b = 5;

    println!("数値 a = {}, b = {}", a, b);

    println!("加算: {} + {} = {}", a, b, calculator::add(a, b));
    println!("減算: {} - {} = {}", a, b, calculator::subtract(a, b));
    println!("乗算: {} * {} = {}", a, b, calculator::multiply(a, b));

    match calculator::divide(a, b) {
        Ok(result) => println!("除算: {} / {} = {}", a, b, result),
        Err(err) => println!("除算エラー: {}", err),
    }

    // 0で割る場合のテスト
    match calculator::divide(a, 0) {
        Ok(result) => println!("除算: {} / 0 = {}", a, result),
        Err(err) => println!("除算エラー: {}", err),
    }

    println!("\n--- utilsディレクトリのhelperモジュールの関数を使用 ---");

    // utilsディレクトリのhelperモジュールの関数を使用
    let number = 42;
    println!("{}", helper::format_number(number));
    println!("{} is even: {}", number, helper::is_even(number));
    println!("{} is even: {}", 23, helper::is_even(23));
    println!("{}", helper::greet("Rustプログラマー"));

    let response = Hello {
        message: "Hello World".to_string(),
    };

    Json(response)
}

#[tokio::main]
async fn main() {
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    let app = Router::new().route("/", get(hello_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
