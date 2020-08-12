use std::collections::HashMap;

use doggo_core::dtos::Pupper;
use doggo_core::queries::data_queries::ViewDataQuery;
use serde::ser::{Serialize, Serializer, SerializeMap};

pub struct GenericContext {
    extra: HashMap<String, String>,
    pub title: String,
    pub logged_in: bool,
    pub has_database: bool,
    pub database_is_working: bool,
}

impl GenericContext {
    pub fn with_title(title: impl ToString) -> Self {
        Self{
            extra: HashMap::new(),
            title: title.to_string(),
            logged_in: false,
            has_database: doggo_infra::CLIENT_POOL.is_configured(),
            database_is_working: doggo_infra::CLIENT_POOL.is_working(),
        }
    }

    pub fn insert(&mut self, key: impl AsRef<str>, value: impl ToString) -> Option<String> {
        match key.as_ref() {
            "title" | "logged_in" | "has_database" => None,
            k => self.extra.insert(k.to_string(), value.to_string())
        }
    }
}

impl Serialize for GenericContext {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_map(Some(4 + self.extra.len()))?;
        state.serialize_entry("title", &self.title)?;
        state.serialize_entry("logged_in", &self.logged_in)?;
        state.serialize_entry("has_database", &self.has_database)?;
        state.serialize_entry("database_is_working", &self.database_is_working)?;
        for (key, value) in self.extra.iter() {
            state.serialize_entry(key, value)?;
        }
        state.end()
    }
}

/// PupperContext provides a context object for templating that contains all the data about a pupper, as well as a boolean
/// indicating that we are logged in, which is required for access to puppers.
#[derive(Serialize, Deserialize)]
pub struct PupperContext {
    pub pupper_id: u64,
    pub name: String,
    pub image: String,
    pub rating: Option<f64>,
    pub logged_in: bool,
    pub has_database: bool,
    pub database_is_working: bool,
}

impl From<Pupper> for PupperContext {
    fn from(p: Pupper) -> Self {
        Self {
            pupper_id: p.id,
            name: p.name,
            image: p.image,
            rating: p.rating,
            logged_in: true,
            has_database: doggo_infra::CLIENT_POOL.is_configured(),
            database_is_working: doggo_infra::CLIENT_POOL.is_working(),
        }
    }
}

/// PupperContext provides a context object for templating that contains all the data about a list of puppers, as well as a boolean
/// indicating that we are logged in, which is required for access to puppers.
#[derive(Serialize, Deserialize)]
pub struct PuppersContext {
    pub puppers: Vec<Pupper>,
    pub logged_in: bool,
}

impl From<Vec<Pupper>> for PuppersContext {
    fn from(puppers: Vec<Pupper>) -> Self {
        Self {
            puppers,
            logged_in: true,
        }
    }
}

/// ViewDataContext provides a context object for templating the view-data page; it contains all the queries we want to be able to run via the front end.
/// This should generally be a static construct.
#[derive(Serialize)]
pub struct ViewDataContext {
    pub queries: &'static [ViewDataContextQuery],
    pub query_id: Option<usize>,
    pub query_result: Option<Vec<Vec<String>>>,
    pub has_database: bool,
    pub database_is_working: bool,
}

impl ViewDataContext {
    pub fn new() -> Self {
        Self{
            queries: VIEW_DATA_QUERIES,
            query_id: None,
            query_result: None,
            has_database: doggo_infra::CLIENT_POOL.is_configured(),
            database_is_working: doggo_infra::CLIENT_POOL.is_working(),
        }
    }

    pub fn with_result(id: usize, result: Vec<Vec<String>>) -> Self {
        let mut context = Self::new();
        context.query_id = Some(id);
        context.query_result = Some(result);
        context
    }
}

#[derive(Serialize)]
pub struct ViewDataContextQuery {
    // TODO:  It probably makes sense for database to be an Option
    pub database: &'static str,
    pub query: &'static str,
}

impl ViewDataContextQuery {
    pub const fn new(database: &'static str, query: &'static str) -> Self {
        Self{database, query}
    }
}

impl From<&ViewDataContextQuery> for ViewDataQuery {
    fn from(other: &ViewDataContextQuery) -> Self {
        Self{
            database: other.database,
            query: other.query,
        }
    }
}

pub const VIEW_DATA_QUERIES: &'static [ViewDataContextQuery] =  &[
    ViewDataContextQuery::new("puppers", "SELECT id, CAST(name AS CHAR) FROM puppers"),
    ViewDataContextQuery::new("puppers:-80", "SELECT id, CAST(name AS CHAR) FROM puppers"),
    ViewDataContextQuery::new("puppers:80-", "SELECT id, CAST(name AS CHAR) FROM puppers"),
];

