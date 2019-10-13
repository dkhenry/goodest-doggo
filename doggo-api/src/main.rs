#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// Uncomment for local development
// use dotenv::dotenv;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery, GetTopTenPuppersQuery};
use doggo_api::Rating;
use domain_patterns::command::Handles;
use doggo_api::generate::{command_handler, query_handler};
use doggo_core::dtos::Puppers;
use doggo_infra::errors::Error as DbError;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/puppers")
}

#[put("/rating", data="<rating>")]
fn rate_pupper(rating: Form<Rating>) -> Result<&'static str,Status> {
    match command_handler().handle(rating.0.into()) {
        Ok(_) => Ok("Success"),
        Err(e) => {
            if let DbError::UniqueViolation = e {
                return Err(Status::Conflict)
            } else {
                return Err(Status::InternalServerError)
            }
        },
    }
}

#[get("/puppers")]
fn get_rando_pupper() -> Result<Template,Status> {
    let pupper = query_handler().handle(GetRandomPupperQuery)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

#[get("/puppers?<id>")]
fn get_puppers(id: u64) -> Result<Template,Status> {
    let pupper = query_handler().handle(GetPupperQuery { id, })
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

#[get("/topten")]
fn top_ten() -> Result<Template,Status> {
    let puppers = query_handler().handle(GetTopTenPuppersQuery)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("topten", Puppers::new(puppers)))
}

fn main() {
//    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index,get_puppers,get_rando_pupper,rate_pupper,top_ten])
        .launch();
}
