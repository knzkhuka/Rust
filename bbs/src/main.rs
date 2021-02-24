use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use actix_web::{http::header, post};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}
impl ResponseError for MyError {}

struct MessageEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<MessageEntry>,
}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failsed to get the connection pool.rusqlite");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table `todo`.");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_message)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id,text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(MessageEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[post("/add")]
async fn add_message(
    params: web::Form<AddParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

// RELETE FROM todo WHERE id BETWEEN 1 AND 100