use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use rusqlite::{Connection, Result, params};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Mutex;
use actix_files::NamedFile;


struct AppState {
    db: Mutex<Connection>,
}

#[derive(serde::Deserialize)]
struct FormData {
    content: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open("pastebin.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pastes (
            token TEXT PRIMARY KEY,
            content TEXT
        )",
        params![],
    ).expect("Failed to create table");

    let app_state = web::Data::new(AppState {
        db: Mutex::new(conn),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::resource("/style.css").to(|| async {
                    NamedFile::open("src/style.css")
                })
            )
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/paste/{token}", web::get().to(get_paste))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
}

async fn submit(data: web::Form<FormData>, state: web::Data<AppState>) -> impl Responder {
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO pastes (token, content) VALUES (?1, ?2)",
        params![token, data.content],
    ).expect("Failed to insert paste");

    HttpResponse::SeeOther()
        .header("Location", format!("/paste/{}", token))
        .finish()

}

async fn get_paste(web::Path(token): web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT content FROM pastes WHERE token = ?1").unwrap();
    let content: String = stmt.query_row(params![token], |row| row.get(0)).unwrap();

    HttpResponse::Ok().body(content)
}