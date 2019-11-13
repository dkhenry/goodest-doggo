#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod generate;

use doggo_core::commands::{RatePupperCommand, CreateUserCommand, LoginCommand};

#[derive(FromForm)]
pub struct Rating {
    pub pupper_id: u64,
    pub rating: u64,
    pub user_id: String,
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

impl Into<RatePupperCommand> for Rating {
    fn into(self) -> RatePupperCommand {
        RatePupperCommand {
            pupper_id: self.pupper_id,
            rating: self.rating,
            user_id: self.user_id,
        }
    }
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
