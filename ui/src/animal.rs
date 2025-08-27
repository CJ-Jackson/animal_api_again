use crate::api::animal::{add_animal, edit_animal, fetch_all_animals, fetch_animal_by_id};
use crate::ext::ResetSignal;
use crate::model::animal::{AnimalModel, AnimalModelSignal};
use dioxus::document::Title;
use dioxus::prelude::*;
use shared::validation::models::animal::AnimalValidationError;
use shared::validation::types::description::DescriptionError;
use shared::validation::types::species::SpeciesError;
use std::sync::Arc;

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
    let animal_input = use_signal(|| AnimalModel::default());
    let mut animal_value = use_signal(|| AnimalModel::default());
    let mut animal_error = use_signal(|| Option::<AnimalValidationError>::None);

    let submit = move |_| async move {
        let animal = animal_input.cloned();
        match animal.validate() {
            Ok(animal_validated) => {
                add_animal(animal_validated.into()).await;
                animal_value.reset();
                animals.restart();
            }
            Err(error) => {
                animal_value.set(animal.merge_with_validate_error(&error));
                animal_error.set(Some(error));
            }
        }
    };

    let animal_error_clone = animal_error.cloned().unwrap_or_default();
    let animal_value_clone = animal_value.cloned();

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
        form { class: "form", onsubmit: submit,
            AnimalFormBody { animal_value: animal_value_clone, animal_input: animal_input,
                animal_validation_error: animal_error_clone }
            button { class: "btn btn-skyblue", type: "submit", "Add"}
        }
    }
}

#[component]
pub fn EditAnimal(id: i64) -> Element {
    let animal = use_resource(move || async move { fetch_animal_by_id(id).await });
    let mut animal_input = use_signal(|| AnimalModel::default());
    let mut animal_value = use_signal(|| AnimalModel::default());
    let mut animal_error = use_signal(|| Option::<AnimalValidationError>::None);

    let animal_error_clone = animal_error.cloned();

    if animal_error_clone.is_none() {
        animal_input.set(animal.cloned().unwrap_or_default());
        animal_value.set(animal.cloned().unwrap_or_default());
    }

    let submit = move |_| async move {
        let animal = animal_input.cloned();
        match animal.validate() {
            Ok(animal_validated) => {
                edit_animal(id, animal_validated.into()).await;
                animal_error.reset();
                navigator().push(Route::Animal {});
            }
            Err(error) => {
                animal_value.set(animal.merge_with_validate_error(&error));
                animal_error.set(Some(error));
            }
        }
    };

    let animal_error_clone = animal_error_clone.unwrap_or_default();
    let animal_value_clone = animal_value.cloned();

    rsx! {
        Title { "Edit Animal" }
        h1 { "Edit Animal" }
        form { class: "form", onsubmit: submit,
            AnimalFormBody { animal_value: animal_value_clone, animal_input: animal_input,
                animal_validation_error: animal_error_clone }
            button { class: "btn btn-skyblue", type: "submit", "Edit"}
        }
        Link { class: "btn btn-skyblue inline-block", to: Route::Animal { }, "Back to Animal" }
    }
}

#[component]
pub fn ErrorMessage(msgs: Arc<[String]>) -> Element {
    rsx! {
        ul { class: "error",
            for msg in msgs.iter() {
                li { class: "error-item", "{msg}" }
            }
        }
    }
}

#[component]
pub fn AnimalFormBody(
    animal_value: AnimalModel,
    mut animal_input: Signal<AnimalModel>,
    animal_validation_error: AnimalValidationError,
) -> Element {
    rsx! {
        label { class:"form-label", r#for: "species", "Species" }
        input { class:"form-item", type: "text", placeholder: "Species",
            name: "species", id: "species", value: animal_value.species,
            oninput: move |e| {
                animal_input.species(e.value());
            }
        }
        if let Err(SpeciesError(msgs)) = animal_validation_error.species {
            ErrorMessage { msgs }
        }
        label { class:"form-label", r#for: "description", "Description" }
        input { class:"form-item", type: "text", placeholder: "Description",
            name: "description", id: "description", value: animal_value.description,
            oninput: move |e| {
                animal_input.description(e.value());
            }
        }
        if let Err(DescriptionError(msgs)) = animal_validation_error.description {
            ErrorMessage { msgs }
        }
    }
}
