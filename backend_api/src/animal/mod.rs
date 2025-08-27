pub mod object;
pub mod repository;
pub mod response;

use crate::ApiTag;
use crate::animal::object::AnimalAddUpdateObject;
use crate::animal::repository::AnimalRepository;
use crate::animal::response::{
    AddAnimalResponse, FetchAllAnimalsResponse, FetchAnimalByIdResponse, UpdateAnimalResponse,
};
use crate::common::context::Dep;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

pub struct AnimalApi;

#[OpenApi(prefix_path = "/animal", tag = "ApiTag::Animal")]
impl AnimalApi {
    /// Fetch All Animals
    #[oai(path = "/", method = "get")]
    async fn index(
        &self,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> FetchAllAnimalsResponse {
        match animal_repository.fetch_all_animals() {
            Ok(animal) => FetchAllAnimalsResponse::Ok(Json(animal.to_vec())),
            Err(_) => FetchAllAnimalsResponse::InternalServerError,
        }
    }

    /// Fetch Animal By ID
    #[oai(path = "/fetch/:id", method = "get")]
    async fn fetch_by_id(
        &self,
        Path(id): Path<u64>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> FetchAnimalByIdResponse {
        match animal_repository.fetch_animal_by_id(id as i64) {
            Ok(animal) => FetchAnimalByIdResponse::Ok(Json(animal)),
            Err(_) => FetchAnimalByIdResponse::NotFound,
        }
    }

    /// Add Animal
    #[oai(path = "/add", method = "post")]
    async fn add(
        &self,
        Json(animal): Json<AnimalAddUpdateObject>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> AddAnimalResponse {
        if animal.to_validate().is_err() {
            return AddAnimalResponse::UnprocessableEntity;
        }
        match animal_repository.add_animal(&animal) {
            Ok(_) => AddAnimalResponse::Created,
            Err(_) => AddAnimalResponse::BadRequest,
        }
    }

    /// Update Animal
    #[oai(path = "/update/:id", method = "patch")]
    async fn update_animal(
        &self,
        Path(id): Path<u64>,
        Json(animal): Json<AnimalAddUpdateObject>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> UpdateAnimalResponse {
        if animal.to_validate().is_err() {
            return UpdateAnimalResponse::UnprocessableEntity;
        }
        match animal_repository.update_animal(&animal, id as i64) {
            Ok(_) => UpdateAnimalResponse::Ok,
            Err(_) => UpdateAnimalResponse::NotFound,
        }
    }
}
