use domain_patterns::command::Handles;
use crate::collection_abstractions::BallotRepository;
use crate::commands::RatePupperCommand;

pub struct VitessPupperCommandHandler<T>
    where T: BallotRepository
{
    repo: T,
}

impl<T> VitessPupperCommandHandler<T>
    where T: BallotRepository
{
    pub fn new(repo: T) -> VitessPupperCommandHandler<T> {
        VitessPupperCommandHandler {
            repo,
        }
    }
}

impl<T> Handles<RatePupperCommand> for VitessPupperCommandHandler<T>
    where T: BallotRepository
{
    type Result = Result<(), T::Error>;

    fn handle(&mut self, msg: RatePupperCommand) -> Self::Result {
        self.repo.insert(&msg.into())
    }
}
