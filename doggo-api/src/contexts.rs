use doggo_core::dtos::Pupper;

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