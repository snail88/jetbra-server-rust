use std::error::Error;

use actix_web::{App, HttpResponse, HttpServer, web};
use base64::Engine;
use base64::engine::general_purpose;
use mime;
use rand::Rng;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::RsaPrivateKey;
use rsa::signature::{SignatureEncoding, Signer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha1::Sha1;
use x509_parser::pem::parse_x509_pem;

const INDEX_HTML: &[u8] = include_bytes!("../assets/index.html");
const ICONS: &[u8] = include_bytes!("../assets/images/icons.svg");

const CRT_PEM: &[u8] = include_bytes!("../jetbra.pem");
const RSA_PRIVATE_KEY: &'static str = include_str!("../jetbra.key");

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub async fn generate_license(web::Json(mut license): web::Json<License>) -> Result<HttpResponse, Box<dyn Error>> {

    // 1 generate license id
    let mut rng = rand::thread_rng();
    let license_id: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    license.license_id = Some(license_id.clone());

    // 2 generate license_part_base64
    let license_part = serde_json::to_string(&license)?;
    let license_part_base64 = general_purpose::STANDARD.encode(license_part.as_bytes());


    // 3 generate sigResultsBase64
    let private_key = RsaPrivateKey::from_pkcs1_pem(RSA_PRIVATE_KEY)?;
    let signing_key = SigningKey::<Sha1>::new(private_key);
    let signature = signing_key.try_sign(license_part.as_bytes())?;
    let sig_results_base64 = general_purpose::STANDARD.encode(signature.to_bytes());


    // 4 generate cert base64
    let (_, pem) = parse_x509_pem(CRT_PEM)?;
    let cert_base64 = general_purpose::STANDARD.encode(pem.contents);

    // 5 combine license
    let license = format!("{}-{}-{}-{}", license_id, license_part_base64, sig_results_base64, cert_base64);

    Ok(HttpResponse::Ok().content_type(mime::APPLICATION_JSON).body(json!({"license": license}).to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().content_type(mime::TEXT_HTML_UTF_8).body(INDEX_HTML) }))
            .route("/index.html", web::get().to(|| async { HttpResponse::Ok().content_type(mime::TEXT_HTML_UTF_8).body(INDEX_HTML) }))
            .route("/images/icons.svg", web::get().to(|| async { HttpResponse::Ok().content_type(mime::IMAGE_SVG).body(ICONS) }))
            .route("/generateLicense", web::post().to(generate_license))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}


#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "licenseId")]
    license_id: Option<String>,

    #[serde(default = "default_licensee_name",rename = "licenseeName")]
    licensee_name: String,

    #[serde(default = "default_empty_str",rename = "assigneeName")]
    assignee_name: String,

    #[serde(default = "default_empty_str",rename = "assigneeEmail")]
    assignee_email: String,

    #[serde(default = "default_empty_str",rename = "licenseRestriction")]
    license_restriction: String,

    #[serde(default = "default_false",rename = "checkConcurrentUse")]
    check_concurrent_use: bool,

    products: Vec<Product>,

    #[serde(default = "default_metadata")]
    metadata: String,

    #[serde(default = "default_hash")]
    hash: String,

    #[serde(default = "default_grace_period_days",rename = "gracePeriodDays")]
    grace_period_days: i32,

    #[serde(default = "default_true",rename = "autoProlongated")]
    auto_prolongated: bool,

    #[serde(default = "default_true",rename = "isAutoProlongated")]
    is_auto_prolongated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    code: String,
    #[serde(default = "default_expire_date",rename = "fallbackDate")]
    fallback_date: String,
    #[serde(default = "default_expire_date",rename = "paidUpTo")]
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
    "2030-12-31".into()
}