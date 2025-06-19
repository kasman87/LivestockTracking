use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[post("/gps")]
async fn receive_gps(body: String) -> impl Responder {
    println!("Received GPS data: {}", body);
    HttpResponse::Ok().body("GPS received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(receive_gps))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
