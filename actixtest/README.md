# actix-web

Rustのactix-webパッケージを使用して簡単なWebサーバを構築してみます。

## 1. プロジェクト準備

```bash
$ cargo new actixtest
$ cd actixtest
$ cargo add actix-web
$ cargo add serde --features derive
```

コマンドを実行すると以下のようなCargo.tomlファイルができあがります。

##### [Cargo.toml](Cargo.toml)

```
[package]
name = "actixtest"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4.4.0"
serde = { version = "1.0.193", features = ["derive"] }
```

## 2. src/main.rs編集

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
        })
        .bind(("127.0.0.1", 3000))?
        .run();
    println!("Serving on http://localhost:3000/");

    server.await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <form action="/" method="post">
                <input type="text" name="name" placeholder="Name">
                <input type="submit">
            </form>
        "#)
}

#[post("/")]
async fn greet(web::Form(form): web::Form<Info>) ->  impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!(r#"
            <h1>Hello, {}!</h1>
        "#, form.name))
}
```
