use poem_openapi::Object;

#[derive(Debug, Object)]
pub struct Message {
    pub message: String,
}
