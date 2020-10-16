#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

// Uncomment for local development
use std::env;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::http::ext::IntoOwned;
use rocket::http::uri::Absolute;
use rocket::request::{Form, FromRequest};
use rocket::response::{Redirect, Flash};
use rocket_contrib::templates::Template;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetRandomPupperQuery, GetPupperQuery, GetTopTenPuppersQuery};
use doggo_api::{execute_view_data_query, Rating, LoginOrSignup, ViewData};
use doggo_api::contexts::{PuppersContext, ViewDataContext, VIEW_DATA_QUERIES};
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
fn configure(flash: Option<FlashMessage>) -> Template {
    let mut context = GenericContext::with_title("Configure Database");
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("configure-database", context)
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

#[post("/authenticate", data = "<input>")]
fn handle_login(
    mut cookies: Cookies,
    input: Form<LoginOrSignup>,
) -> Result<Redirect, Flash<Redirect>> {
    let result = match input.0.action.as_ref() {
        "Login" => user_command_handler().handle(LoginCommand::from(input.0)),
        "Sign Up" => user_command_handler().handle(CreateUserCommand::from(input.0)),
        _ => return Err(Flash::error(
            Redirect::to(uri!(login)),
            "Invalid action. Please try again."
        ))
    };
    match result {
        Ok(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id));
            Ok(Redirect::to(uri!(index)))
        },
        Err(e) => {
            println!("{:?}", e);
            if let CoreError::NotAuthorized = e {
                Err(Flash::error(
                    Redirect::to(uri!(login)),
                    "Invalid email/password.",
                ))
            } else {
                println!("{:?}", e);
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

#[derive(Debug)]
struct ConfigureRequest {
    referer: Option<String>
}

impl<'a, 'r> FromRequest<'a, 'r> for ConfigureRequest {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        rocket::Outcome::Success(Self{
            referer: request.headers().get_one("Referer").map(|s| s.to_string())
        })
    }
}

#[post("/configure", data = "<database_url>")]
fn handle_configure(req: ConfigureRequest, database_url: Form<Configure>) -> Flash<Redirect> {
    let url = database_url.0.database_url.trim();
    // If the database wasn't already configured, redirect to signup
    // If no Referer request header was set, or if it was invalid, redirect to login
    // Otherwise, redirect to whatever page sent the request
    let redirect_target = match doggo_infra::CLIENT_POOL.is_configured() {
        true => match req.referer {
            Some(v) => match Absolute::parse(&v) {
                Ok(uri) => uri.origin().unwrap().clone().into_owned(),
                Err(_) => uri!(login)
            },
            None => uri!(login)
        },
        false => uri!(login)
    };
    match doggo_infra::CLIENT_POOL.set_url(url) {
        Ok(_) => Flash::success(Redirect::to(redirect_target), format!("Database URL set to: {}", url)),
        // TODO:  Nicer error messages than "UrlError { URL ParseError { relative URL without a base } }"
        Err(e) => Flash::error(Redirect::to(uri!(configure)), format!("Unable to set database URL: {}", e))
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
fn get_rando_pupper(_user_id: UserId) -> Result<Template, Status> {
    match query_handler().handle(GetRandomPupperQuery) {
        Ok(Some(pupper)) => Ok(Template::render("pupper", PupperContext::from(pupper))),
        Ok(None) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("{}", e);
            Ok(Template::render("pupper", GenericContext::with_title("Rando Pupper")))
        }
    }
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

#[get("/view-data")]
fn view_data(_user_id: UserId) -> Result<Template, Status> {
    let context;
    if VIEW_DATA_QUERIES.len() > 0 {
        context = execute_view_data_query(0)?;
    } else {
        context = ViewDataContext::new()
    }
    Ok(Template::render("view-data", context))
}

#[get("/view-data", rank = 2)]
fn view_data_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

#[post("/view-data", data = "<query>")]
fn view_data_result(query: Form<ViewData>, _user_id: UserId) -> Result<Template, Status> {
    if query.0.query_id >= VIEW_DATA_QUERIES.len() {
        return Err(Status::NotFound);
    }
    Ok(Template::render("view-data", execute_view_data_query(query.0.query_id)?))
}

#[post("/view-data", rank = 2)]
fn view_data_result_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

fn main() {
    dotenv().ok();

    if !cfg!(debug_assertions) {
        let dir = {
            let mut path = env::current_exe().unwrap();
            path.pop();
            path
        };
        env::set_current_dir(dir).unwrap();
    }

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![configure,handle_configure,login,handle_login,logout,index,puppers_redirect,get_puppers,get_rando_pupper,rate_pupper,top_ten,top_ten_redirect, view_data, view_data_redirect, view_data_result, view_data_result_redirect])
        .launch();
}
