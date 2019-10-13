use crate::collection_abstractions::PupperRepository;
use domain_patterns::query::HandlesQuery;
use crate::queries::pupper_queries::{GetPupperQuery, GetRandomPupperQuery, GetTopTenPuppersQuery};
use crate::dtos::Pupper;

pub struct VitessPupperQueriesHandler<T>
    where T: PupperRepository
{
    repo: T,
}

impl<T> VitessPupperQueriesHandler<T>
    where T: PupperRepository
{
    /// Associative function to create a new query handler from a connection.
    pub fn new(repo: T) -> VitessPupperQueriesHandler<T> {
        VitessPupperQueriesHandler {
            repo,
        }
    }
}

impl<T: PupperRepository> HandlesQuery<GetPupperQuery> for VitessPupperQueriesHandler<T> {
    type Result = Result<Option<Pupper>, T::Error>;

    fn handle(&mut self, query: GetPupperQuery) -> Self::Result {
        self.repo.get(query.id)
    }
}

impl<T> HandlesQuery<GetRandomPupperQuery> for VitessPupperQueriesHandler<T>
    where T: PupperRepository
{
    type Result = Result<Option<Pupper>, T::Error>;

    #[allow(unused_variables)]
    fn handle(&mut self, query: GetRandomPupperQuery) -> Self::Result {
        self.repo.get_random()
    }
}

impl<T> HandlesQuery<GetTopTenPuppersQuery> for VitessPupperQueriesHandler<T>
    where T: PupperRepository
{
    type Result = Result<Option<Vec<Pupper>>, T::Error>;

    #[allow(unused_variables)]
    fn handle(&mut self, query: GetTopTenPuppersQuery) -> Self::Result {
        self.repo.get_top_ten()
    }
}
