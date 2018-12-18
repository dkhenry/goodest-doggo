#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate rocket;
use rocket::http::{RawStr,Status};
use rocket::http::uri::Uri;
use rocket::request::Form;
use rocket::response::Redirect;

#[macro_use] extern crate rocket_contrib;
use rocket_contrib::databases::mysql;

use rocket_contrib::templates::Template;

#[database("doggers")]
struct DoggersDb(mysql::Conn);

#[derive(Serialize)]
struct Pupper {
    id: u64,
    name: String,
    image: String,
    rating: f64
}

fn newPupper() -> Pupper {
    Pupper {
        id: 0,
        name: String::from(""),
        image: String::from(""),
        rating: 0.0,
    }
}
           
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

fn pupper_rating(mut conn:DoggersDb, name: &str) -> f64 {
    for row in conn.0.query(format!("SELECT COALESCE(SUM(r.rating)/COUNT(r.rating),0.0) FROM doggers@replica.ratings as r WHERE r.pupper_name='{}'",name)).unwrap() {
        let r: f64 = mysql::from_row(row.unwrap());
        return r
    }
    return 0.0
}
#[get("/puppers")]
fn get_rando_pupper(mut conn: DoggersDb) -> Result<Template,Status> {
    let mut p: Pupper = newPupper();   
    for row in conn.0.query(format!("SELECT p._id, p.name, p.image FROM doggers@replica.puppers AS p ORDER BY RAND() LIMIT 1")).unwrap() {
        let r: (u64,String,String) = mysql::from_row(row.unwrap());
        p.id = r.0;
        p.name = r.1;
        p.image = r.2            
    }
    p.rating = pupper_rating(conn, &p.name);
    return Ok(Template::render("pupper",p))
}

#[get("/puppers?<name>")]
fn get_puppers(mut conn: DoggersDb, name: &RawStr) -> Result<Template,Status> {
    for row in conn.0.query(format!("SELECT _id, name, image FROM puppers WHERE name = '{}'",Uri::percent_decode(name.as_bytes()).unwrap())).unwrap() {
        let r: (u64,String,String) = mysql::from_row(row.unwrap());
        return Ok(Template::render("pupper",Pupper{
            id: r.0,
            name: r.1,
            image: r.2,
            rating: 0.0,
        }))
    }   
    return Err(Status::NotFound)
}

fn main() {
    rocket::ignite()
        .attach(DoggersDb::fairing())
        .attach(Template::fairing())        
        .mount("/", routes![index,get_puppers,get_rando_pupper,rate_pupper])
        .launch();
}
