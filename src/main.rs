mod handlers;
mod models;
mod storage;

use actix_web::{web, App, HttpServer};
use tera::Tera;
use dotenv::dotenv;
use tokio;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Инициализация хранилища
    let storage = storage::Storage::new(
        "redis://127.0.0.1/",
        "sqlite:pastes.db"
    ).await;

    // Запуск фонового таска
    let storage_clone = storage.clone();
    tokio::spawn(async move {
        cleanup_task(storage_clone).await;
    });

    // Загрузка шаблонов
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storage.clone()))
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(handlers::index))
            .route("/pastes", web::post().to(handlers::create_paste))
            .route("/pastes/{id}", web::get().to(handlers::view_paste))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn cleanup_task(_storage: storage::Storage) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(86400)).await;
        // Логика очистки будет добавлена позже
    }
}