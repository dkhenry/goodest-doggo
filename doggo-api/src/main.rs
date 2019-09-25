#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use dotenv::dotenv;
use rocket_contrib::databases::mysql;
use rocket_contrib::templates::Template;
use doggo_infra::query_handlers::VitessPupperQueriesHandler;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery};
use doggo_api::Rating;
use doggo_infra::command_handlers::VitessPupperCommandHandler;
use domain_patterns::command::Handles;

#[database("doggers")]
struct DoggersDb(mysql::Conn);


#[get("/")]
fn index() -> Redirect {
    Redirect::to("/puppers")
}

#[put("/rating", data="<rating>")]
fn rate_pupper(conn: DoggersDb, rating: Form<Rating>) -> Result<&'static str,Status> {
    let mut command_handler = VitessPupperCommandHandler::new(conn.0);
    match command_handler.handle(rating.0.into()) {
        Ok(_) => Ok("Success"),
        // TODO: Improve error handling.  This might not always mean the user messed up.
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/puppers")]
fn get_rando_pupper(conn: DoggersDb) -> Result<Template,Status> {
    let mut query_handler = VitessPupperQueriesHandler::new(conn.0);
    let pupper = query_handler.handle(GetRandomPupperQuery)
        // Map underlying database error to 500
        .map_err(|_|Status::InternalServerError)?
        // Map None to 404
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

#[get("/puppers?<name>")]
fn get_puppers(conn: DoggersDb, name: String) -> Result<Template,Status> {
    let mut query_handler = VitessPupperQueriesHandler::new(conn.0);
    let pupper = query_handler.handle(GetPupperQuery { name, })
        // Map underlying database error to 500
        .map_err(|_| Status::InternalServerError)?
        // Map None to 404
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",pupper))
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DoggersDb::fairing())
        .attach(Template::fairing())        
        .mount("/", routes![index,get_puppers,get_rando_pupper,rate_pupper])
        .launch();
}
