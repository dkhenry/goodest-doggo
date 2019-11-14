use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::ballot::Ballot;
use ulid::Ulid;

pub mod pupper_commands_handler;
pub mod user_commands_handler;

#[derive(Command)]
pub struct RatePupperCommand {
    pub pupper_id: u64,
    pub rating: u64,
    pub user_id: String,
}

#[derive(Command)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
}

#[derive(Command)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

impl Into<Ballot> for RatePupperCommand {
    fn into(self) -> Ballot {
        Ballot {
            rating: self.rating,
            pupper_id: self.pupper_id,
            user_id: self.user_id,
        }
    }
}