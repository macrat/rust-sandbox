use actix_web::{
    get,
    http::header,
    post,
    web::{Data, Form, Path},
    App, HttpResponse, HttpServer, Result,
};
use askama::Template;
use serde::Deserialize;

mod db;

use db::Database;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    notes: Vec<db::Note>,
}

#[get("/")]
async fn index(db: Data<Database>) -> Result<HttpResponse> {
    let notes = match db.get_all().await {
        Ok(notes) => notes,
        Err(err) => {
            eprintln!("failed to fetch data: {}", err);
            return Ok(HttpResponse::InternalServerError().body("failed to fetch data"));
        }
    };

    let s = IndexTemplate { notes }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Deserialize)]
struct Register {
    text: String,
}

#[post("/register")]
async fn register(db: Data<Database>, form: Form<Register>) -> Result<HttpResponse> {
    if let Err(err) = db.register(&form.text).await {
        eprintln!("failed to insert data: {}", err);
        return Ok(HttpResponse::InternalServerError().body("failed to insert data"));
    };

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[get("/{id}/remove")]
async fn remove(db: Data<Database>, params: Path<i64>) -> Result<HttpResponse> {
    if let Err(err) = db.remove(params.into_inner()).await {
        eprintln!("failed to remove data: {}", err);
        return Ok(HttpResponse::InternalServerError().body("failed to remove data"));
    };

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::connect("db.sqlite").await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(()); // XXX
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .service(index)
            .service(register)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
