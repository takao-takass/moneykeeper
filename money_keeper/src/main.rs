use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

async fn get_sample_data() -> impl Responder {
    HttpResponse::Ok().json("chinpo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .route("/samples", web::get().to(get_sample_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
