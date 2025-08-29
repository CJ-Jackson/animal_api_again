use crate::api::{ApiClientError, get_client, get_url};
use crate::model::animal::{AnimalAddUpdateModel, AnimalModel};
use error_stack::{Report, ResultExt};

pub fn default_animals() -> Vec<AnimalModel> {
    let mut v: Vec<AnimalModel> = vec![];
    v.push(AnimalModel {
        id: 0,
        species: "Lion".to_string(),
        description: "King of the Jungle".to_string(),
    });
    v
}

pub async fn fetch_all_animals() -> Result<Vec<AnimalModel>, Report<ApiClientError>> {
    let client = get_client();
    let req = client
        .get(format!("{}/animal", get_url()))
        .build()
        .change_context(ApiClientError)?;

    let res = client.execute(req).await.change_context(ApiClientError)?;
    Ok(res
        .json::<Vec<AnimalModel>>()
        .await
        .change_context(ApiClientError)?)
}

pub async fn fetch_animal_by_id(id: i64) -> Result<AnimalModel, Report<ApiClientError>> {
    let client = get_client();
    let req = client
        .get(format!("{}/animal/fetch/{}", get_url(), id))
        .build()
        .change_context(ApiClientError)?;

    let res = client.execute(req).await.change_context(ApiClientError)?;
    Ok(res
        .json::<AnimalModel>()
        .await
        .change_context(ApiClientError)?)
}

pub async fn add_animal(animal: AnimalAddUpdateModel) -> Result<(), Report<ApiClientError>> {
    let client = get_client();
    let req = client
        .post(format!("{}/animal/add", get_url()))
        .json(&animal)
        .build()
        .change_context(ApiClientError)?;

    _ = client.execute(req).await.change_context(ApiClientError)?;
    Ok(())
}

pub async fn edit_animal(
    id: i64,
    animal: AnimalAddUpdateModel,
) -> Result<(), Report<ApiClientError>> {
    let client = get_client();
    let req = client
        .patch(format!("{}/animal/update/{}", get_url(), id))
        .json(&animal)
        .build()
        .change_context(ApiClientError)?;

    _ = client.execute(req).await.change_context(ApiClientError)?;
    Ok(())
}
