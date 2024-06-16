mod utils;
use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

const FILE_SIZE: usize = 14000000;

#[derive(Deserialize, Debug)]
struct PostData {
    title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    image: String,
}

// function zone start
fn store_to_file<T: AsRef<[u8]>>(file_name: &str, data: T) -> Result<(), HttpResponse> {
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into());
        }
    };
    if let Err(e) = file.write_all(data.as_ref()) {
        eprintln!("Failed to write to file: {}", e);
        return Err(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into());
    }

    Ok(())
}
// function zone end

#[post("/post")]
async fn post(data: web::Json<PostData>) -> HttpResponse {
    let title_len = data.title.len();
    let text_len = data.text.len();
    let img_len = data.image.len();

    // Check size
    if title_len <= 0
        || title_len > 50
        || text_len <= 0
        || text_len > 500
        || img_len <= 0
        || img_len > FILE_SIZE
    {
        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).into();
    }

    // Write data to file
    if let Err(response) = store_to_file("./data/title.txt", data.title.clone()) {
        return response;
    }
    if let Err(response) = store_to_file("./data/text.txt", data.text.clone()) {
        return response;
    }
    if let Err(response) = store_to_file("./data/image.txt", data.image.clone()) {
        return response;
    }

    HttpResponse::Ok().finish()
}

#[get("/")]
async fn get_html() -> impl Responder {
    let html = utils::html::generate_html();
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_html).service(post))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}