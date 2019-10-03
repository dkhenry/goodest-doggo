use doggo_infra::handlers::command_handlers::VitessPupperCommandHandler;
use doggo_infra::handlers::query_handlers::VitessPupperQueriesHandler;
use doggo_infra::handlers::VitessQueryRetryWrapper;

pub fn command_handler() -> VitessPupperCommandHandler {
    VitessPupperCommandHandler::new()
}

pub fn query_handler() -> VitessQueryRetryWrapper {
    let main_handler = VitessPupperQueriesHandler::new();
    VitessQueryRetryWrapper::new(main_handler)
}