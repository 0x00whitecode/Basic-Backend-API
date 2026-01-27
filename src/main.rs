use actix_files::{NamedFile, Files};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};


async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listen on the port 8080");
    HttpServer::new(|| {
        App::new()
        .service(
            Files::new("/static", "static")
            .show_files_listing(),
        )
        .service(
            Files::new("/image", "image")
            .show_files_listing(),
        )
            .route("/hell", web::get().to(hello))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}