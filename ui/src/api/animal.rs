use crate::api::{get_client, get_url};
use crate::model::animal::{AnimalAddUpdateModel, AnimalModel};

fn default_animals() -> Vec<AnimalModel> {
    let mut v: Vec<AnimalModel> = vec![];
    v.push(AnimalModel {
        id: 0,
        species: "Lion".to_string(),
        description: "King of the Jungle".to_string(),
    });
    v
}

pub async fn fetch_all_animals() -> Vec<AnimalModel> {
    let client = get_client();
    let req = client.get(format!("{}/animal", get_url())).build();
    let req = match req {
        Ok(req) => req,
        Err(_) => return default_animals(),
    };

    let res = client.execute(req).await;
    match res {
        Ok(res) => {
            let output = res.json::<Vec<AnimalModel>>().await;
            output.unwrap_or_else(|_| default_animals())
        }
        Err(_) => default_animals(),
    }
}

pub async fn fetch_animal_by_id(id: i64) -> AnimalModel {
    let client = get_client();
    let req = client
        .get(format!("{}/animal/fetch/{}", get_url(), id))
        .build();
    let req = match req {
        Ok(req) => req,
        Err(_) => return AnimalModel::default(),
    };

    let res = client.execute(req).await;
    match res {
        Ok(res) => {
            let output = res.json::<AnimalModel>().await;
            output.unwrap_or_else(|_| AnimalModel::default())
        }
        Err(_) => AnimalModel::default(),
    }
}

pub async fn add_animal(animal: AnimalAddUpdateModel) {
    let client = get_client();
    let req = client
        .post(format!("{}/animal/add", get_url()))
        .json(&animal)
        .build();

    match req {
        Ok(req) => {
            _ = client.execute(req).await;
        }
        Err(_) => {}
    }
}

pub async fn edit_animal(id: i64, animal: AnimalAddUpdateModel) {
    let client = get_client();
    let req = client
        .patch(format!("{}/animal/update/{}", get_url(), id))
        .json(&animal)
        .build();

    match req {
        Ok(req) => {
            _ = client.execute(req).await;
        }
        Err(_) => {}
    }
}
