use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::ballot::Ballot;

pub mod pupper_commands_handler;

#[derive(Command)]
pub struct RatePupperCommand {
    pub pupper_id: u64,
    pub rating: u64,
}

impl Into<Ballot> for RatePupperCommand {
    fn into(self) -> Ballot {
        Ballot {
            rating: self.rating,
            pupper_id: self.pupper_id,
        }
    }
}