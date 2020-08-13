#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

pub mod generate;
pub mod contexts;
pub use contexts::*;

use domain_patterns::query::HandlesQuery;
use rocket::http::Status;
use doggo_core::commands::{RatePupperCommand, CreateUserCommand, LoginCommand};
use crate::{ViewDataContext, VIEW_DATA_QUERIES};
use crate::generate::data_query_handler;

#[derive(FromForm)]
pub struct Rating {
    pub pupper_id: u64,
    pub rating: u64,
}

impl Rating {
    pub fn into_rate_pupper_cmd(self, user_id: String) -> RatePupperCommand {
        RatePupperCommand {
            pupper_id: self.pupper_id,
            rating: self.rating,
            user_id
        }
    }
}

#[derive(FromForm)]
pub struct LoginOrSignup {
    pub email: String,
    pub password: String,
    pub action: String,
}

impl From<LoginOrSignup> for CreateUserCommand {
    fn from(value: LoginOrSignup) -> Self {
        Self{
            email: value.email,
            password: value.password
        }
    }
}

impl From<LoginOrSignup> for LoginCommand {
    fn from(value: LoginOrSignup) -> Self {
        Self{
            email: value.email,
            password: value.password
        }
    }
}

#[derive(FromForm)]
pub struct ViewData {
    pub query_id: usize,
}

pub fn execute_view_data_query(id: usize) -> Result<ViewDataContext, Status> {
    let query = &VIEW_DATA_QUERIES[id];
    match data_query_handler().handle(query.into()) {
        Ok(result) => Ok(ViewDataContext::with_result(id, result)),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

