#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct PasteForm {
    pub content: String,
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct Paste {
    pub id: String,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}