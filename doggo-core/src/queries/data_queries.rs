use domain_patterns::query::Query;

#[derive(Query)]
pub struct ViewDataQuery {
    pub query: &'static str,
}

