use poem_openapi::Object;

#[derive(Debug, Object, Clone)]
pub struct AnimalObject {
    pub id: i64,
    pub species: String,
    pub description: String,
}

#[derive(Debug, Object)]
pub struct AnimalAddUpdateObject {
    pub species: String,
    pub description: String,
}
