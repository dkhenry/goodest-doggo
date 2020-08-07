use std::collections::HashMap;

use doggo_core::dtos::Pupper;
use serde::ser::{Serialize, Serializer, SerializeMap};

pub struct GenericContext {
    extra: HashMap<String, String>,
    pub title: String,
    pub logged_in: bool,
    pub has_database: bool,
}

impl GenericContext {
    pub fn with_title(title: impl ToString) -> Self {
        Self{
            extra: HashMap::new(),
            title: title.to_string(),
            logged_in: false,
            has_database: doggo_infra::CLIENT_POOL.is_configured(),
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
        let mut state = serializer.serialize_map(Some(3 + self.extra.len()))?;
        state.serialize_entry("title", &self.title)?;
        state.serialize_entry("logged_in", &self.logged_in)?;
        state.serialize_entry("has_database", &self.has_database)?;
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
}

impl From<Pupper> for PupperContext {
    fn from(p: Pupper) -> Self {
        Self {
            pupper_id: p.id,
            name: p.name,
            image: p.image,
            rating: p.rating,
            logged_in: true,
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
