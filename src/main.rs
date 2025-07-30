use axum::{routing::get, serve, Router};
use rust_server::math::calculator;
use rust_server::utils::helper;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
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

            "Hello World"
        }),
    );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
