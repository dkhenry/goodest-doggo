use domain_patterns::query::Query;

#[derive(Query)]
pub struct ViewDataQuery {
    pub database: &'static str,
    pub query: &'static str,
}

