use doggo_core::queries::pupper_query_handler::VitessPupperQueriesHandler;
use doggo_infra::repositories::{VitessPupperRepository, VitessBallotRepository, PupperRepositoryRetryWrapper};
use doggo_core::commands::pupper_commands_handler::VitessPupperCommandHandler;

pub fn command_handler() -> VitessPupperCommandHandler<VitessBallotRepository> {
    let repo = VitessBallotRepository::new();
    VitessPupperCommandHandler::new(repo)
}

pub fn query_handler() -> VitessPupperQueriesHandler<PupperRepositoryRetryWrapper<VitessPupperRepository>> {
    let repo = VitessPupperRepository::new();
    let wrapped_repo = PupperRepositoryRetryWrapper::new(repo);
    let handler = VitessPupperQueriesHandler::new(wrapped_repo);
    handler
}