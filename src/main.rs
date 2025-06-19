use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};

// GET route for the homepage
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("🚜 Livestock Tracking Server is running")
}

// POST route for /gps
#[post("/gps")]
async fn receive_gps(form: web::Form<std::collections::HashMap<String, String>>) -> impl Responder {
    if let (Some(lat), Some(lon)) = (form.get("lat"), form.get("lon")) {
        println!("📡 Received GPS: lat={}, lon={}", lat, lon);
        HttpResponse::Ok().body("GPS received")
    } else {
        HttpResponse::BadRequest().body("Missing lat or lon")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(receive_gps)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
