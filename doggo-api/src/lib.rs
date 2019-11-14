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
pub struct Signup {
    pub email: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct Login {
    pub email: String,
    pub password: String,
}


impl Into<CreateUserCommand> for Signup {
    fn into(self) -> CreateUserCommand {
        CreateUserCommand {
            email: self.email,
            password: self.password,
        }
    }
}

impl Into<LoginCommand> for Login {
    fn into(self) -> LoginCommand {
        LoginCommand {
            email: self.email,
            password: self.password,
        }
    }
}
