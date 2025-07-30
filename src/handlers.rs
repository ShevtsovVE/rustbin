use crate::models;
use crate::storage;

use actix_web::{web, HttpResponse};
use tera::Tera;

pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
    let rendered = tera.render("index.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn create_paste(
    form: web::Form<models::PasteForm>,
    storage: web::Data<storage::Storage>,
) -> HttpResponse {
    let id = storage.save_paste(form.content.clone()).await;
    HttpResponse::SeeOther()
        .append_header(("Location", format!("/pastes/{}", id)))
        .finish()
}

pub async fn view_paste(
    path: web::Path<String>,
    storage: web::Data<storage::Storage>,
    tera: web::Data<Tera>,
) -> HttpResponse {
    let id = path.into_inner();
    if let Some(content) = storage.get_paste(&id).await {
        let mut ctx = tera::Context::new();
        ctx.insert("content", &content);
        ctx.insert("id", &id);  // Добавляем ID в контекст
        match tera.render("paste.html", &ctx) {
            Ok(rendered) => HttpResponse::Ok().body(rendered),
            Err(e) => {
                eprintln!("Ошибка рендеринга: {}", e);
                HttpResponse::InternalServerError().body("Ошибка рендеринга шаблона")
            }
        }
    } else {
        HttpResponse::NotFound().body("Paste not found")
    }
}