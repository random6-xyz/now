mod utils;
use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;
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

// TODO: @random6 check if it works
#[derive(Deserialize, Debug, PartialEq)]
enum GetQueryRole {
    Friend,
    Family,
    Random,
}

#[derive(Deserialize, Debug)]
struct GetQuery {
    role: GetQueryRole,
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

// TODO: @random6 Create ENV variable in OS
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

// TODO: @random6 Add auth logic, URL
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
        Some(GetQueryRole::Family) => html = utils::html::generate_html(GetQueryRole::Family),
        Some(GetQueryRole::Friend) => html = utils::html::generate_html(GetQueryRole::Friend),
        Some(GetQueryRole::Random) | None => {
            html = utils::html::generate_html(GetQueryRole::Random)
        }
    }

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_html).service(post))
        .bind(("0.0.0.0", 7777))?
        .workers(5)
        .run()
        .await
}
