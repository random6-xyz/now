mod utils;
use actix_web::{
    get, http::StatusCode, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger;
use log::info;
use serde::Deserialize;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

const FILE_SIZE: usize = 14000000;

#[derive(Deserialize, Debug)]
struct PostData {
    title: String,
    text: String,
    image: String,
    role: String,
    session: String,
}

#[derive(Deserialize, Debug, PartialEq, Default)]
enum GetQueryRole {
    Friend,
    Family,
    #[default]
    Random,
}

#[derive(Deserialize, Debug)]
struct GetQuery {
    #[serde(default)]
    role: GetQueryRole,
    #[serde(default)]
    session: String,
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

fn get_session_from_env(role: GetQueryRole) -> String {
    match role {
        GetQueryRole::Family => {
            env::var("NOW_FAMILY_SESSION").expect("No env named NOW_FAMILY_SESSION")
        }
        GetQueryRole::Friend => {
            env::var("NOW_FRIEND_SESSION").expect("No env named NOW_FRIEND_SESSION")
        }
        GetQueryRole::Random => String::new(),
    }
}

fn check_session_role(query: web::Query<GetQuery>) -> Option<GetQueryRole> {
    if query.role == GetQueryRole::Family
        && query.session == get_session_from_env(GetQueryRole::Family)
    {
        return Some(GetQueryRole::Family);
    } else if query.role == GetQueryRole::Friend
        && query.session == get_session_from_env(GetQueryRole::Friend)
    {
        return Some(GetQueryRole::Friend);
    } else if query.role == GetQueryRole::Random {
        return Some(GetQueryRole::Random);
    } else {
        return None;
    }
}
// function zone end

#[post("/post")]
async fn post(data: web::Json<PostData>) -> HttpResponse {
    let role = data.role.clone();
    let session = data.session.clone();
    let title_len = data.title.len();
    let text_len = data.text.len();
    let img_len = data.image.len();

    // Check role, session
    if session != env::var("NOW_ADMIN_SESSION").expect("No env named NOW_ADMIN_SESSION") {
        return HttpResponse::build(StatusCode::UNAUTHORIZED).into();
    }

    if !["family", "friend", "random"].contains(&role.as_str()) {
        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).into();
    }

    // Check size
    if title_len > 50 || text_len > 500 || img_len > FILE_SIZE {
        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).into();
    }

    // Log
    info!(
        "\nPosted - {}\ntitle:{}\ntext: {}\n",
        role, &data.title, &data.text
    );

    // Write data to file
    if let Err(response) = store_to_file(
        format!("./data/{}/title.txt", role).as_str(),
        data.title.clone(),
    ) {
        return response;
    }
    if let Err(response) = store_to_file(
        format!("./data/{}/text.txt", role).as_str(),
        data.text.clone(),
    ) {
        return response;
    }
    if let Err(response) = store_to_file(
        format!("./data/{}/image.txt", role).as_str(),
        data.image.clone(),
    ) {
        return response;
    }

    HttpResponse::Ok().finish()
}

#[get("/")]
async fn get_html(get_query: web::Query<GetQuery>) -> impl Responder {
    let html: String;
    match check_session_role(get_query) {
        Some(GetQueryRole::Family) => {
            html = utils::html::generate_html(GetQueryRole::Family);
            info!("\nGet - family\n");
        }
        Some(GetQueryRole::Friend) => {
            html = utils::html::generate_html(GetQueryRole::Friend);
            info!("\nGet - friend\n");
        }
        Some(GetQueryRole::Random) | None => {
            html = utils::html::generate_html(GetQueryRole::Random);
            info!("\nGet - random\n");
        }
    }

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(get_html)
            .service(post)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 80))?
    .workers(2)
    .run()
    .await
}
