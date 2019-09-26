#[derive(Serialize, Deserialize)]
pub struct Pupper {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub rating: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Puppers {
    pub puppers: Vec<Pupper>
}

impl Puppers {
    pub fn new(puppers: Vec<Pupper>) -> Puppers {
        Puppers {
            puppers,
        }
    }
}