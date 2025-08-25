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
    async fn index(&self, animal_repository: Dep<AnimalRepository>) -> FetchAllAnimalsResponse {
        match animal_repository.0.fetch_all_animals() {
            Ok(animal) => FetchAllAnimalsResponse::Ok(Json(animal.to_vec())),
            Err(_) => FetchAllAnimalsResponse::InternalServerError,
        }
    }

    /// Fetch Animal By ID
    #[oai(path = "/fetch/:id", method = "get")]
    async fn fetch_by_id(
        &self,
        Path(id): Path<u64>,
        animal_repository: Dep<AnimalRepository>,
    ) -> FetchAnimalByIdResponse {
        match animal_repository.0.fetch_animal_by_id(id as i64) {
            Ok(animal) => FetchAnimalByIdResponse::Ok(Json(animal)),
            Err(_) => FetchAnimalByIdResponse::NotFound,
        }
    }

    /// Add Animal
    #[oai(path = "/add", method = "post")]
    async fn add(
        &self,
        animal: Json<AnimalAddUpdateObject>,
        animal_repository: Dep<AnimalRepository>,
    ) -> AddAnimalResponse {
        match animal_repository.0.add_animal(&animal.0) {
            Ok(_) => AddAnimalResponse::Created,
            Err(_) => AddAnimalResponse::BadRequest,
        }
    }

    /// Update Animal
    #[oai(path = "/update/:id", method = "patch")]
    async fn update_animal(
        &self,
        Path(id): Path<u64>,
        animal: Json<AnimalAddUpdateObject>,
        animal_repository: Dep<AnimalRepository>,
    ) -> UpdateAnimalResponse {
        match animal_repository.0.update_animal(&animal.0, id as i64) {
            Ok(_) => UpdateAnimalResponse::Ok,
            Err(_) => UpdateAnimalResponse::NotFound,
        }
    }
}
