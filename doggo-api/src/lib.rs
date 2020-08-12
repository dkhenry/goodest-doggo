#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

pub mod generate;
pub mod contexts;
pub use contexts::*;

use doggo_core::commands::{RatePupperCommand, CreateUserCommand, LoginCommand};

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

