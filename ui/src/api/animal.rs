use crate::api::{get_client, get_url};
use crate::model::animal::{Animal, AnimalAddUpdate};

fn default_animals() -> Vec<Animal> {
    let mut v: Vec<Animal> = vec![];
    v.push(Animal {
        id: 0,
        species: "Lion".to_string(),
        description: "King of the Jungle".to_string(),
    });
    v
}

pub async fn fetch_all_animals() -> Vec<Animal> {
    let client = get_client();
    let req = client.get(format!("{}/animal", get_url())).build();
    let req = match req {
        Ok(req) => req,
        Err(e) => return default_animals(),
    };

    let res = client.execute(req).await;
    match res {
        Ok(res) => {
            let output = res.json::<Vec<Animal>>().await;
            output.unwrap_or_else(|_| default_animals())
        }
        Err(_) => default_animals(),
    }
}

pub async fn add_animal(animal: AnimalAddUpdate) {
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
