use crate::collection_abstractions::DataRepository;
use domain_patterns::query::HandlesQuery;
use crate::queries::data_queries::ViewDataQuery;

pub struct VitessDataQueryHandler<T: DataRepository> {
    repo: T,
}

impl<T: DataRepository> VitessDataQueryHandler<T> {
    /// Associative function to create a new query handler from a connection.
    pub fn new(repo: T) -> Self {
        Self{
            repo,
        }
    }
}

impl<T: DataRepository> HandlesQuery<ViewDataQuery> for VitessDataQueryHandler<T> {
    type Result = Result<Vec<Vec<String>>, T::Error>;

    fn handle(&mut self, query: ViewDataQuery) -> Self::Result {
        self.repo.get(query.database, query.query)
    }
}

