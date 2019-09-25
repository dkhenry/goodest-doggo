#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use dotenv::dotenv;
use rocket_contrib::templates::Template;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery};
use doggo_api::Rating;
use domain_patterns::command::Handles;
use doggo_api::generate::{command_handler, query_handler};

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/puppers")
}

#[put("/rating", data="<rating>")]
fn rate_pupper(rating: Form<Rating>) -> Result<&'static str,Status> {
    match command_handler().handle(rating.0.into()) {
        Ok(_) => Ok("Success"),
        // TODO: Improve error handling.  This might not always mean the user messed up.
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/puppers")]
fn get_rando_pupper() -> Result<Template,Status> {
    let pupper = query_handler().handle(GetRandomPupperQuery)
        // Map underlying database error to 500
        .map_err(|_|Status::InternalServerError)?
        // Map None to 404
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

#[get("/puppers?<name>")]
fn get_puppers(name: String) -> Result<Template,Status> {
    let pupper = query_handler().handle(GetPupperQuery { name, })
        // Map underlying database error to 500
        .map_err(|_| Status::InternalServerError)?
        // Map None to 404
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index,get_puppers,get_rando_pupper,rate_pupper])
        .launch();
}
