#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate rocket;

use doggo_core::commands::RatePupperCommand;

#[derive(FromForm)]
pub struct Rating {
    pub name: String,
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
