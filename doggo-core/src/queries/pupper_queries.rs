use domain_patterns::query::Query;

#[derive(Query)]
pub struct GetPupperQuery {
    pub name: String,
}

#[derive(Query)]
pub struct GetRandomPupperQuery;

#[derive(Query)]
pub enum PupperQueries {
    GetPupper(GetPupperQuery),
    GetRandomPupper(GetRandomPupperQuery),
}

impl From<GetPupperQuery> for PupperQueries {
    fn from(query: GetPupperQuery) -> Self {
        PupperQueries::GetPupper(query)
    }
}

impl From<GetRandomPupperQuery> for PupperQueries {
    fn from(query: GetRandomPupperQuery) -> Self {
        PupperQueries::GetRandomPupper(query)
    }
}
