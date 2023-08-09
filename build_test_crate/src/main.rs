use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine as _};
use std::{
    fs::File,
    io::BufReader,
    io::{self, BufRead},
    path::Path,
    str,
};
include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

static HOST_PATH: &[u8] = "https://sapes5.sapdevcenter.com/sap/opu/odata/iwbep".as_bytes();
static SERVICE_NAME: &[u8] = "GWSAMPLE_BASIC".as_bytes();

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn fetch_auth() -> Result<String, String> {
    let mut user = String::from("unknown");
    let mut pwd = String::from("unknown");

    // Try to obtain userid and password from environment variable file .env
    if let Ok(lines) = read_lines(".env") {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.starts_with("SAP_USER") {
                        let (_, u) = l.split_at(l.find("=").unwrap() + 1);
                        user = u.to_owned();
                    }
                    if l.starts_with("SAP_PASSWORD") {
                        let (_, p) = l.split_at(l.find("=").unwrap() + 1);
                        pwd = p.to_owned();
                    }
                },
                Err(_) => (),
            }
        }
    }

    if user.eq("unknown") || pwd.eq("unknown") {
        Err("SAP userid and/or password missing from .env file".to_owned())
    } else {
        Ok(general_purpose::STANDARD.encode(format!("{}:{}", user, pwd)))
    }
}

#[get("/")]
async fn fetch_entity_set() -> impl Responder {
    let client = reqwest::Client::new();
    let ent_set_name = GwsampleBasicEntities::BusinessPartnerSet.value();

    match fetch_auth() {
        Ok(auth_chars) => {
            match client
                .get(format!(
                    "{}/{}/{}?$format=json",
                    str::from_utf8(HOST_PATH).unwrap(),
                    str::from_utf8(SERVICE_NAME).unwrap(),
                    ent_set_name
                ))
                //        .header("Authorization", "Basic UDIwMDEzODU3ODk6VHJhbnF1aWwlMGNlYW4=")
                .header("Authorization", format!("Basic {}", auth_chars))
                .send()
                .await
                .expect("Here's an SAP-style error message: Ein Fehler ist aufgetreten. Viel Glück 🤣🤣🤣")
                .text()
                .await
            {
                Ok(res) => HttpResponse::Ok().content_type(ContentType::json()).body(res),
                Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
            }
        },
        Err(err) => HttpResponse::BadRequest().body(format!("{:#?}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fetch_entity_set))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
