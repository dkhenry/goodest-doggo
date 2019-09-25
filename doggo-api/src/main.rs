#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;

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

#[database("doggers")]
struct DoggersDb(mysql::Conn);

#[derive(FromForm)]
struct Rating {
    name: String,
    rating: u64,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/puppers")
}

#[put("/rating", data="<rating>")]
fn rate_pupper(mut conn: DoggersDb, rating: Form<Rating>) -> Result<&'static str,Status> {
    match conn.0.query(format!("INSERT INTO ratings (pupper_name, rating ) VALUES ( '{}','{}' )",rating.name,rating.rating)) {
        Ok(_) => Ok("Success"),
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
