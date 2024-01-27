use std::error::Error;
use axum::{
    routing::get,
    Router,
    response::Json,
    response::Html,
};
use axum::http::header;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
const INDEX_HTML: &[u8] = include_bytes!("../assets/index.html");
const ICONS: &[u8] = include_bytes!("../assets/images/icons.svg");




async fn generate_license(Json(payload): Json<License>) -> Json<Value> {
    dbg!(payload);
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(|| async { Html(INDEX_HTML) }))
        .route("/images/icons.svg", get(|| async { ([(header::CONTENT_TYPE, "image/svg+xml")], ICONS) }))
        .route("/generateLicense", post(generate_license));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    return Ok(());
}



#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    license_id: Option<String>,

    #[serde(default = "default_licensee_name")]
    licensee_name: String,

    #[serde(default = "default_empty_str")]
    assignee_name: String,

    #[serde(default = "default_empty_str")]
    assignee_email: String,

    #[serde(default = "default_empty_str")]
    license_restriction: String,

    #[serde(default = "default_false")]
    check_concurrent_use: bool,

    products: Vec<Product>,

    #[serde(default = "default_metadata")]
    metadata: String,

    #[serde(default = "default_hash")]
    hash: String,

    #[serde(default = "default_grace_period_days")]
    grace_period_days: i32,

    #[serde(default = "default_true")]
    auto_prolongated: bool,

    #[serde(default = "default_true")]
    is_auto_prolongated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    code: String,
    #[serde(default = "default_expire_date")]
    fallback_date: String,
    #[serde(default = "default_expire_date")]
    paid_up_to: String,
    #[serde(default = "default_true")]
    extended: bool,
}


fn default_licensee_name() -> String {
    String::from("for test only")
}
fn default_empty_str() -> String {
    String::from("")
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_metadata() -> String {
    "0120230102PPAA013009".into()
}

fn default_hash() -> String {
    "41472961/0:1563609451".into()
}

fn default_grace_period_days() -> i32 {
    7
}

fn default_expire_date() -> String {
    "2023-12-31".into()
}