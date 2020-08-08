#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

// Uncomment for local development
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{Form, FromRequest};
use rocket::response::{Redirect, Flash};
use rocket_contrib::templates::Template;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery, GetTopTenPuppersQuery};
use doggo_api::{Rating, Signup, Login, PuppersContext};
use domain_patterns::command::Handles;
use doggo_api::generate::{pupper_command_handler, query_handler, user_command_handler};
use doggo_api::contexts::{GenericContext, PupperContext};
use doggo_infra::errors::Error as DbError;
use doggo_core::errors::Error as CoreError;
use rocket::Request;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::FlashMessage;
use rocket::request;
use doggo_core::commands::{LoginCommand, CreateUserCommand};

macro_rules! connection_check {
    () => {{
        if !doggo_infra::CLIENT_POOL.is_configured() {
            return Err(Flash::error(
                Redirect::to(uri!(configure)),
                "Database is not configured"
            ));
        }
    }}
}

struct UserId(String);

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserId, !> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserId(id))
            .or_forward(())
    }
}

#[get("/configure")]
fn configure() -> Template {
    let context = GenericContext::with_title("Configure Database");
    Template::render("configure-database", context)
}

#[get("/signup")]
fn signup() -> Result<Template, Flash<Redirect>> {
    connection_check!();
    let context = GenericContext::with_title("Sign Up");
    Ok(Template::render("login-signup", context))
}

#[get("/login")]
fn login_user(_user: UserId) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login(flash: Option<FlashMessage>) -> Result<Template, Flash<Redirect>> {
    connection_check!();
    let mut context = GenericContext::with_title("Login");

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Ok(Template::render("login-signup", context))
}

#[post("/login", data = "<user>")]
fn handle_login(
    mut cookies: Cookies,
    user: Form<Login>,
) -> Result<Redirect, Flash<Redirect>> {
    let login_cmd: LoginCommand = user.0.into();
    match user_command_handler().handle(login_cmd) {
        Ok(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id));
            Ok(Redirect::to(uri!(index)))
        },
        Err(e) => {
            if let CoreError::NotAuthorized = e {
                Err(Flash::error(
                    Redirect::to(uri!(login)),
                    "Invalid email/password.",
                ))
            } else {
                Err(Flash::error(
                    Redirect::to(uri!(login)),
                    "Internal server error. Please try again.",
                ))
            }
        }
    }
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login)), "Successfully logged out.")
}

#[derive(FromForm)]
struct Configure {
    database_url: String,
}

#[post("/configure", data = "<database_url>")]
fn handle_configure(
    database_url : Form<Configure>,
) -> Flash<Redirect> {
    doggo_infra::CLIENT_POOL.set_url(&database_url.0.database_url);
    Flash::success(Redirect::to(uri!(login)), format!("Database URL set to: {}",database_url.0.database_url))
}

#[post("/signup", data = "<user>")]
fn handle_signup(
    mut cookies: Cookies,
    user: Form<Signup>,
) -> Result<Redirect, Flash<Redirect>> {
    let create_user_cmd: CreateUserCommand = user.0.into();
    match user_command_handler().handle(create_user_cmd) {
        Ok(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id));
            Ok(Redirect::to(uri!(index)))
        },
        Err(e) => {
            println!("{}", e);
            Err(Flash::error(
                Redirect::to(uri!(login)),
                "Internal server error. Please try again.",
            ))
        }
    }
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/puppers")
}

#[put("/rating", data="<rating>")]
fn rate_pupper(rating: Form<Rating>, user_id: UserId) -> Result<Redirect, Status> {
    let cmd = rating.0.into_rate_pupper_cmd(user_id.0);
    match pupper_command_handler().handle(cmd) {
        Ok(_) => {
            Ok(Redirect::to(uri!(get_rando_pupper)))
        },
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
fn get_rando_pupper(_user_id: UserId) -> Result<Template,Status> {
    let pupper = query_handler().handle(GetRandomPupperQuery)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",PupperContext::from(pupper)))
}

#[get("/puppers", rank = 2)]
fn puppers_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/puppers?<id>")]
fn get_puppers(id: u64, _user_id: UserId) -> Result<Template,Status> {
    let pupper = query_handler().handle(GetPupperQuery { id, })
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("pupper",PupperContext::from(pupper)))
}

#[get("/topten")]
fn top_ten(_user_id: UserId) -> Result<Template,Status> {
    let puppers = query_handler().handle(GetTopTenPuppersQuery)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("topten", PuppersContext::from(puppers)))
}

#[get("/topten", rank = 2)]
fn top_ten_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![configure,handle_configure,login,signup,handle_signup,handle_login,logout,index,puppers_redirect,get_puppers,get_rando_pupper,rate_pupper,top_ten,top_ten_redirect])
        .launch();
}
