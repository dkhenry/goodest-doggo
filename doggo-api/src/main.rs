#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use dotenv::dotenv;
use rocket_contrib::templates::Template;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery, GetTopTenPuppersQuery};
use doggo_api::Rating;
use domain_patterns::command::Handles;
use doggo_api::generate::{command_handler, query_handler};
use doggo_core::dtos::{Pupper, Puppers};

#[get("/")]
fn index() -> Redirect {
//    Redirect::to("/puppers")
    Redirect::to("/test")
}

#[get("/test")]
fn test() -> Result<Template,Status> {
    let fake_pup = Pupper {
        id: 0,
        name: "".to_string(),
        image: "".to_string(),
        rating: None
    };
    Ok(Template::render("pupper", fake_pup))
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
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

#[get("/puppers?<name>")]
fn get_puppers(name: String) -> Result<Template,Status> {
    let pupper = query_handler().handle(GetPupperQuery { name, })
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
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index,get_puppers,get_rando_pupper,rate_pupper,test,top_ten])
        .launch();
}
