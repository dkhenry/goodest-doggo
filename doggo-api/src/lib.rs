#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod generate;

use doggo_core::commands::RatePupperCommand;

#[derive(FromForm)]
pub struct Rating {
    pub pupper_id: u64,
    pub rating: u64,
}

impl Into<RatePupperCommand> for Rating {
    fn into(self) -> RatePupperCommand {
        RatePupperCommand {
            pupper_id: self.pupper_id,
            rating: self.rating,
        }
    }
}
