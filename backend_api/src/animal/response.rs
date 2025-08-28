use crate::animal::object::{AnimalErrorObject, AnimalObject};
use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;

#[derive(ApiResponse)]
pub enum FetchAllAnimalsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<AnimalObject>>),
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
pub enum FetchAnimalByIdResponse {
    #[oai(status = 200)]
    Ok(Json<AnimalObject>),
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum AddAnimalResponse {
    #[oai(status = 201)]
    Created,
    #[oai(status = 422)]
    UnprocessableEntity(Json<AnimalErrorObject>),
    #[oai(status = 400)]
    BadRequest,
}

#[derive(ApiResponse)]
pub enum UpdateAnimalResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 422)]
    UnprocessableEntity(Json<AnimalErrorObject>),
    #[oai(status = 404)]
    NotFound,
}
