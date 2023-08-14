#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::Header;
use rocket::response::status;
use logic::analyze_position;


#[derive(FromForm)]
struct Query {
    pos: Option<String>,
}

#[get("/?<query..>")]
fn index(query: Option<Query>) -> String {
    match query {
        Some(q) => q.pos.unwrap_or_else(|| "Hello, World!".to_string()),
        None => "Hello, World!".to_string(),
    }
}

#[get("/analyze?<query..>")]
fn analyze(query: Option<Query>) -> status::Custom<String> {
    match query {
        Some(q) => {
            let pos = q.pos.unwrap_or_else(|| "Hello, World!".to_string());

            if pos == "-1" {
                return match analyze_position("") {
                    Ok(analysis) => status::Custom(rocket::http::Status::Ok, analysis.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")),
                    Err(e) => status::Custom(rocket::http::Status::BadRequest, e.to_string()),
                };
            }

            // Check if the query string contains only numbers between 1 and 7
            if !pos.chars().all(|c| c.is_digit(10) && c >= '1' && c <= '7') {
                return status::Custom(rocket::http::Status::BadRequest, "Invalid input. Please provide numbers between 1 and 7 or -1.".to_string());
            }

            return match analyze_position(&pos) {
                Ok(analysis) => status::Custom(rocket::http::Status::Ok, analysis.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")),
                Err(e) => status::Custom(rocket::http::Status::BadRequest, e.to_string()),
            };
        }
        None => status::Custom(rocket::http::Status::BadRequest, "Wrong Query Parameter".to_string()),
    }
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .attach(Cors)
        .mount("/", routes![index, analyze]);
    Ok(rocket.into())
}
