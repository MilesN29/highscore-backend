use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HighScore {
    name: String,
    score: u32,
}

async fn get_high_scores() -> impl Responder {
    let high_scores = vec![
        HighScore {
            name: "Alice".to_string(),
            score: 1000,
        },
        HighScore {
            name: "Bob".to_string(),
            score: 800,
        },
    ];
    web::Json(high_scores)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/highscores", web::get().to(get_high_scores))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
