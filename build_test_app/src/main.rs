pub mod auth;
pub mod err_handlers;
pub mod feed;
pub mod xml;

use crate::{auth::fetch_auth, err_handlers::error_handlers};
use feed::Feed;

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    middleware, web, App, Error, HttpResponse, HttpServer, Result,
};
use serde_json::json;
use std::{collections::HashMap, str};
use tinytemplate::TinyTemplate;

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

static INDEX: &str = include_str!("../html/index.html");
static HOST_PATH: &[u8] = "https://sapes5.sapdevcenter.com/sap/opu/odata/iwbep".as_bytes();
static SERVICE_NAME: &[u8] = "GWSAMPLE_BASIC".as_bytes();

// ---------------------------------------------------------------------------------------------------------------------
// Serve document root
// ---------------------------------------------------------------------------------------------------------------------
async fn doc_root(
    tmpl: web::Data<TinyTemplate<'_>>,
    _query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let ctx = json!({
      "service_name": str::from_utf8(SERVICE_NAME).unwrap(),
      "option_list": GwsampleBasicEntities::as_list()
    });

    let body = tmpl
        .render("index.html", &ctx)
        .map_err(|err| error::ErrorInternalServerError(format!("Template error\n{}", err)))?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}

// ---------------------------------------------------------------------------------------------------------------------
// Serve entity set contents
// ---------------------------------------------------------------------------------------------------------------------
#[get("/{entity_set_name}")]
async fn entity_set(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::new();
    let entity_set_name = path.into_inner();

    let http_responce = match fetch_auth() {
        Ok(auth_chars) => {
            match client
                .get(format!(
                    "{}/{}/{}?$top=100",
                    str::from_utf8(HOST_PATH).unwrap(),
                    str::from_utf8(SERVICE_NAME).unwrap(),
                    entity_set_name
                ))
                .header("Authorization", format!("Basic {}", auth_chars))
                .send()
                .await
                .expect("Here's an SAP-style error message: Ein Fehler ist aufgetreten. Viel Glück 🤣🤣🤣")
                .text()
                .await
            {
                Ok(res) => {
                    let response = match Feed::<BusinessPartner>::from_str(&res) {
                        Ok(bp) => format!("{bp:#?}"),
                        Err(e) => format!("Error: {e:?}"),
                    };

                    HttpResponse::Ok().content_type(ContentType::plaintext()).body(response)
                },
                Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
            }
        },
        Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
    };

    Ok(http_responce)
}

// ---------------------------------------------------------------------------------------------------------------------
// Start web server
// ---------------------------------------------------------------------------------------------------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();

        App::new()
            .app_data(web::Data::new(tt))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(doc_root)))
            .service(entity_set)
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
