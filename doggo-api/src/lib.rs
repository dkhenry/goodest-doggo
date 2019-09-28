#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod generate;

use doggo_core::commands::RatePupperCommand;

#[derive(FromForm)]
pub struct Rating {
    pub pup_id: u64,
    pub name: String,
    pub image: String,
    pub rating: u64,
}

impl Into<RatePupperCommand> for Rating {
    fn into(self) -> RatePupperCommand {
        RatePupperCommand {
            name: self.name,
            rating: self.rating,
        }
    }
}
