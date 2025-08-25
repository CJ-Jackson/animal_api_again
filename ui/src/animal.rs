use crate::api::animal::{add_animal, fetch_all_animals};
use crate::model::animal::AnimalAddUpdate;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn Animal() -> Element {
    let mut animals = use_resource(|| async move { fetch_all_animals().await });
    let mut species = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "container content",
            h1 { "Animal" }
            div { class: "animals",
                for animal in animals.cloned().unwrap_or_default().iter() {
                    div { class: "animal-item",
                        span { class: "animal-id", "{animal.id}" }
                        span { class: "animal-other", "{animal.species}" }
                        span { class: "animal-other", "{animal.description}" }
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
                    add_animal(AnimalAddUpdate{
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
}
