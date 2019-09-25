use r2d2_mysql::MysqlConnectionManager;
use domain_patterns::command::Handles;
use doggo_core::commands::RatePupperCommand;

type Conn = r2d2_mysql::r2d2::PooledConnection<MysqlConnectionManager>;

pub struct VitessPupperCommandHandler {
    conn: Conn,
}

impl VitessPupperCommandHandler {
    /// Associative function to create a new command handler from a connection.
    pub fn new(conn: Conn) -> VitessPupperCommandHandler {
        VitessPupperCommandHandler {
            conn,
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
