use crate::api::animal::{add_animal, edit_animal, fetch_all_animals, fetch_animal_by_id};
use crate::model::animal::AnimalAddUpdateModel;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Animal {},
    #[route("/edit/:id")]
    EditAnimal { id: i64 },
}

#[component]
pub fn UiApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "container content",
            Router::<Route> {}
        }
    }
}

#[component]
pub fn Animal() -> Element {
    let mut animals = use_resource(|| async move { fetch_all_animals().await });
    let mut species = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    rsx! {
        h1 { "Animal" }
        div { class: "animals",
            for animal in animals.cloned().unwrap_or_default().iter() {
                div { class: "animal-item",
                    span { class: "animal-id", "{animal.id}" }
                    span { class: "animal-other", "{animal.species}" }
                    span { class: "animal-other", "{animal.description}" }
                    Link { class: "animal-id btn btn-skyblue",
                        to: Route::EditAnimal { id: animal.id }, "Edit" }
                }
            }
        }
        div { class: "form",
            label { class:"form-label", r#for: "species", "Species" }
            input { class:"form-item", type: "text", placeholder: "Species", name: "species",
                oninput: move |e| species.set(e.value()) }
            label { class:"form-label", r#for: "description", "Description" }
            input { class:"form-item", type: "text", placeholder: "Description", name: "description",
                oninput: move |e| description.set(e.value()) }
        }
        div {
            button { class: "btn btn-skyblue", onclick: move |_| async move {
                add_animal(AnimalAddUpdateModel{
                    species: species.to_string(),
                    description: description.to_string(),
                }).await;
                species.set("".to_string());
                description.set("".to_string());
                animals.restart();
            }, id: "Add", "Add" }
        }
    }
}

#[component]
pub fn EditAnimal(id: i64) -> Element {
    let nav = navigator();

    let animal = use_resource(move || async move { fetch_animal_by_id(id).await });
    let mut species = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    rsx! {
        h1 { "Edit Animal" }
        div { class: "form",
            label { class:"form-label", r#for: "species", "Species" }
            input { class:"form-item", type: "text", placeholder: "Species",
                name: "species", value: animal.cloned().unwrap_or_default().species,
                oninput: move |e| species.set(e.value()) }
            label { class:"form-label", r#for: "description", "Description" }
            input { class:"form-item", type: "text", placeholder: "Description",
                name: "description", value: animal.cloned().unwrap_or_default().description,
                oninput: move |e| description.set(e.value()) }
        }
        div {
            button { class: "btn btn-skyblue", onclick: move |_| async move {
                let animal = animal.cloned().unwrap_or_default();
                edit_animal(id, AnimalAddUpdateModel{
                    species: species.to_string().is_empty().then(|| animal.species).unwrap_or(species.to_string()),
                    description: description.to_string().is_empty().then(|| animal.description).unwrap_or(description.to_string()),
                }).await;
                species.set("".to_string());
                description.set("".to_string());
                nav.push(Route::Animal {});
            }, id: "Edit", "Edit" }
        }
        Link { class: "animal-id btn btn-skyblue", to: Route::Animal { }, "Back to Animal" }
    }
}
