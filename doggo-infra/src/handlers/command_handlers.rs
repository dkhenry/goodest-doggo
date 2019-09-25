use domain_patterns::command::Handles;
use doggo_core::commands::RatePupperCommand;
use super::CLIENT_POOL;
use super::Conn;
use mysql;

pub struct VitessPupperCommandHandler {
    conn: Conn,
}

impl VitessPupperCommandHandler {
    /// Associative function to create a new command handler from a connection.
    pub fn new() -> VitessPupperCommandHandler {
        VitessPupperCommandHandler {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            conn: CLIENT_POOL.clone().get_conn().unwrap(),
        }
    }
}

impl Handles<RatePupperCommand> for VitessPupperCommandHandler {
    type Result = Result<(), mysql::Error>;

    fn handle(&mut self, msg: RatePupperCommand) -> Self::Result {
        match self.conn.prep_exec(
            r"INSERT INTO ratings (pupper_name, rating )
            VALUES (?,?)",
            (msg.name, msg.rating,)
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
