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
use crate::common::results::unified;
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
        unified(async {
            animal_repository
                .fetch_all_animals()
                .map(|animal| FetchAllAnimalsResponse::Ok(Json(animal.to_vec())))
                .map_err(|_| FetchAllAnimalsResponse::InternalServerError)
        })
        .await
    }

    /// Fetch Animal By ID
    #[oai(path = "/fetch/:id", method = "get")]
    async fn fetch_by_id(
        &self,
        Path(id): Path<u64>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> FetchAnimalByIdResponse {
        unified(async {
            animal_repository
                .fetch_animal_by_id(id as i64)
                .map(|animal| FetchAnimalByIdResponse::Ok(Json(animal)))
                .map_err(|_| FetchAnimalByIdResponse::NotFound)
        })
        .await
    }

    /// Add Animal
    #[oai(path = "/add", method = "post")]
    async fn add(
        &self,
        Json(animal): Json<AnimalAddUpdateObject>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> AddAnimalResponse {
        unified(async {
            animal.to_validate().map_err(|animal_err| {
                AddAnimalResponse::UnprocessableEntity(Json(animal_err.into()))
            })?;
            animal_repository
                .add_animal(&animal)
                .map(|_| AddAnimalResponse::Created)
                .map_err(|_| AddAnimalResponse::BadRequest)
        })
        .await
    }

    /// Update Animal
    #[oai(path = "/update/:id", method = "patch")]
    async fn update_animal(
        &self,
        Path(id): Path<u64>,
        Json(animal): Json<AnimalAddUpdateObject>,
        Dep(animal_repository): Dep<AnimalRepository>,
    ) -> UpdateAnimalResponse {
        unified(async {
            animal.to_validate().map_err(|animal_error| {
                UpdateAnimalResponse::UnprocessableEntity(Json(animal_error.into()))
            })?;
            animal_repository
                .update_animal(&animal, id as i64)
                .map(|_| UpdateAnimalResponse::Ok)
                .map_err(|_| UpdateAnimalResponse::NotFound)
        })
        .await
    }
}
