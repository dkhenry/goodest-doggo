use domain_patterns::query::Query;

// These queries should always return a Result<Option<Pupper>, dbError>
#[derive(Query)]
pub struct GetPupperQuery {
    pub name: String,
}

#[derive(Query)]
pub struct GetRandomPupperQuery;

// The following are queries that can return a list of results.
#[derive(Query)]
pub struct GetTopTenPuppersQuery;
