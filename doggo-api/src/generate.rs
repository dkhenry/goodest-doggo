use doggo_core::queries::pupper_query_handler::VitessPupperQueriesHandler;
use doggo_core::queries::data_query_handler::VitessDataQueryHandler;
use doggo_infra::repositories::{VitessPupperRepository, VitessBallotRepository, PupperRepositoryRetryWrapper, VitessUserRepository, VitessDataRepository};
use doggo_core::commands::pupper_commands_handler::VitessPupperCommandHandler;
use doggo_core::commands::user_commands_handler::VitessUserCommandHandler;

pub fn pupper_command_handler() -> VitessPupperCommandHandler<VitessBallotRepository> {
    let repo = VitessBallotRepository::new();
    VitessPupperCommandHandler::new(repo)
}

pub fn user_command_handler() -> VitessUserCommandHandler<VitessUserRepository> {
    let repo = VitessUserRepository::new();
    VitessUserCommandHandler::new(repo)
}

pub fn query_handler() -> VitessPupperQueriesHandler<PupperRepositoryRetryWrapper<VitessPupperRepository>> {
    let repo = VitessPupperRepository::new();
    let wrapped_repo = PupperRepositoryRetryWrapper::new(repo);
    let handler = VitessPupperQueriesHandler::new(wrapped_repo);
    handler
}

pub fn data_query_handler() -> VitessDataQueryHandler<VitessDataRepository> {
	let repo = VitessDataRepository::new();
	VitessDataQueryHandler::new(repo)
}

