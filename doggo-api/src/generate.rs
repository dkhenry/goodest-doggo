use doggo_infra::handlers::command_handlers::VitessPupperCommandHandler;
use doggo_infra::handlers::query_handlers::VitessPupperQueriesHandler;

pub fn command_handler() -> VitessPupperCommandHandler {
    VitessPupperCommandHandler::new()
}

pub fn query_handler() -> VitessPupperQueriesHandler {
    VitessPupperQueriesHandler::new()
}