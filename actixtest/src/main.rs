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
