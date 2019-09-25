use domain_patterns::command::Command;
use domain_patterns::message::Message;

#[derive(Command)]
pub struct RatePupperCommand {
    pub name: String,
    pub rating: u64,
}