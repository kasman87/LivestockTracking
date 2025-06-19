use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

// POST handler for GPS
#[post("/gps")]
async fn receive_gps(body: String) -> impl Responder {
    println!("GPS Data: {}", body);
    HttpResponse::Ok().body("GPS received")
}

// ✅ NEW: GET handler for /
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("🐄 Livestock Tracking Webhook is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(receive_gps)
            .service(index) // <- Add this line
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
