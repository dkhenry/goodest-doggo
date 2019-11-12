#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate domain_derive;

pub mod errors;
pub use errors::*;
pub mod queries;
pub mod commands;
pub mod collection_abstractions;
pub mod ballot;
pub mod user;
pub mod value_objects;
pub mod dtos;
