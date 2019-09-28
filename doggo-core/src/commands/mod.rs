use domain_patterns::command::Command;
use domain_patterns::message::Message;

#[derive(Command)]
pub struct RatePupperCommand {
    pub pupper_id: u64,
    pub rating: u64,
}