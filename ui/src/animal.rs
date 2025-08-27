use crate::api::animal::{add_animal, edit_animal, fetch_all_animals, fetch_animal_by_id};
use crate::ext::ResetSignal;
use crate::model::animal::{AnimalAddUpdateModel, AnimalModel, AnimalModelSignal};
use dioxus::document::Title;
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
    let mut animal_input = use_signal(|| AnimalModel::default());
    let mut animal_value = use_signal(|| AnimalModel::default());

    let submit = move || async move {
        let animal = animal_input.cloned();
        add_animal(animal.into()).await;
        animal_value.reset();
        animals.restart();
    };

    rsx! {
        Title { "Animal" }
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
        form { class: "form", onsubmit: move |_| async move { submit().await },
            label { class:"form-label", r#for: "species", "Species" }
            input { class:"form-item", id: "species", type: "text", placeholder: "Species",
                name: "species", value: animal_value.cloned().species,
                oninput: move |e| {
                    animal_input.species(e.value());
                },
            }
            label { class:"form-label", r#for: "description", "Description" }
            input { class:"form-item", id: "description", type: "text", placeholder: "Description",
                name: "description", value: animal_value.cloned().description,
                oninput: move |e| {
                    animal_input.description(e.value());
                },
            }
            button { class: "btn btn-skyblue", type: "submit", "Add"}
        }
    }
}

#[component]
pub fn EditAnimal(id: i64) -> Element {
    let nav = navigator();

    let animal = use_resource(move || async move { fetch_animal_by_id(id).await });
    let mut animal_input = use_signal(|| AnimalModel::default());
    animal_input.set(animal.cloned().unwrap_or_default());

    let submit = move || async move {
        let animal = animal_input.cloned();
        edit_animal(id, animal.into()).await;
        nav.push(Route::Animal {});
    };

    rsx! {
        Title { "Edit Animal" }
        h1 { "Edit Animal" }
        form { class: "form", onsubmit: move |_| async move { submit().await },
            label { class:"form-label", r#for: "species", "Species" }
            input { class:"form-item", type: "text", placeholder: "Species",
                name: "species", id: "species", value: animal.cloned().unwrap_or_default().species,
                oninput: move |e| {
                    animal_input.species(e.value());
                }
            }
            label { class:"form-label", r#for: "description", "Description" }
            input { class:"form-item", type: "text", placeholder: "Description",
                name: "description", id: "description", value: animal.cloned().unwrap_or_default().description,
                oninput: move |e| {
                    animal_input.description(e.value());
                }
            },
            button { class: "btn btn-skyblue", type: "submit", "Edit"}
        }
        Link { class: "btn btn-skyblue inline-block", to: Route::Animal { }, "Back to Animal" }
    }
}
