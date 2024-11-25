use actix_cors::Cors; // For handling cross-origin requests.
use actix_web::{middleware::Logger, web, App, HttpServer, Responder}; // For handling HTTP requests.
use serde::Serialize; // For serializing structs to JSON.

// define a struct for representing a high score.
// `Serialize` allows this struct to be converted into JSON
#[derive(Serialize)]
struct HighScore {
    name: String, // Player's name.
    score: i32,   // Use signed integer to match Postgres' int4 type.
}

// defin an asynchronous handler function for the `/highscores` route.
// the function returns a JSON response with a list of high scores.
async fn get_high_scores() -> impl Responder {
    //load enviroment variables
    dotenv::dotenv().ok();

    //get database connection string from the enviorment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls)
        .await
        .unwrap();

    //spawn a seperate task to manage connection

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let statement = client
        .prepare("SELECT name, score FROM high_scores ORDER BY score DESC LIMIT 10")
        .await
        .unwrap();
    let rows = client.query(&statement, &[]).await.unwrap();

    let high_scores: Vec<HighScore> = rows
        .into_iter()
        .map(|row| HighScore {
            name: row.get("name"),
            score: row.get("score"),
        })
        .collect();

    web::Json(high_scores) // return the high scores as JSON.
}

// server is configured and started.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://milesn29.github.io/game")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials(); // If you're using cookies or authentication

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/highscores", web::get().to(get_high_scores))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
