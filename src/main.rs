use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use serde::Serialize;


#[derive(Serialize)]
pub struct RespondeJson {
    pub success:String,
    pub message:String,
}
#[derive(Serialize)]
pub struct Identity {
    pub name:String,
}

async fn health_check () -> impl Responder {
    let res = vec![
        RespondeJson {
            success: "true".to_string(),
            message: "API is running".to_string()
        }
    ];
    return web::Json(res);
}

async fn welcome_message () -> impl Responder{
    let res = vec![
        RespondeJson {
            success: "true".to_string(),
            message: "welcome to HNG Task 0x0001".to_string()
        } 
    ];
    return web::Json(res);
}

async fn welcome_message_with_name (req: HttpRequest) -> impl Responder{
   let  name = req.match_info().get("name").unwrap_or("world");
   let res = vec![
    Identity{
        name: name.to_string() + " we have your detail now",
    }
   ]; 
   
   return web::Json(res);
   

}



// build reusable configuration
fn api_config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
        .route("/", web::get().to(welcome_message))
        .route("/{name}", web::get().to(welcome_message_with_name))
        .route("/health", web::get().to(health_check))
        //welcome_message_with_name

    );
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", bind_addr);
    HttpServer::new(|| {
        App::new()
       .configure(api_config)
       
    })
    .bind(&bind_addr)?
    .run()
    .await
}



