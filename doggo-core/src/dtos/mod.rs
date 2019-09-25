#[derive(Serialize, Deserialize)]
pub struct Pupper {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub rating: Option<f64>,
}