use crate::api::animal::{add_animal, edit_animal, fetch_all_animals, fetch_animal_by_id};
use crate::common::locale::{LocaleForStore, build_locale_config};
use crate::ext::ResetSignal;
use crate::model::animal::{AnimalModel, AnimalModelSignal};
use cjtoolkit_structured_validator::common::locale::ValidateErrorStore;
use cjtoolkit_structured_validator::types::description::DescriptionError;
use dioxus::document::Title;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_primitives::alert_dialog::*;
use shared::validation::models::animal::{AnimalValidated, AnimalValidationError};
use shared::validation::types::species::SpeciesError;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Animal {},
    #[route("/edit/:id")]
    EditAnimal { id: i64 },
    #[route("/error")]
    ErrorPage {},
}

#[component]
pub fn UiApp() -> Element {
    use_init_i18n(|| build_locale_config());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "container content",
            Router::<Route> {}
        }
    }
}

#[component]
pub fn ErrorPage() -> Element {
    rsx! {
        Title { "Error" }
        h1 { "Error" }
        p { "Unable to fetch data" }
        Link { class: "btn btn-skyblue", to: Route::Animal {}, "Restart" }
    }
}

#[component]
pub fn Animal() -> Element {
    let mut animals = use_resource(|| async move {
        fetch_all_animals().await.unwrap_or_else(|_| {
            navigator().push(Route::ErrorPage {});
            vec![]
        })
    });
    let animal_input = use_signal(|| AnimalModel::default());
    let mut animal_value = use_signal(|| AnimalModel::default());
    let mut animal_validated = use_signal(|| AnimalValidated::default());
    let mut animal_error = use_signal(|| Option::<AnimalValidationError>::None);
    let mut open = use_signal(|| false);

    let alert = move |e: Event<FormData>| {
        e.prevent_default();
        async move {
            let animal = animal_input.cloned();
            match animal.validate() {
                Ok(animal_validated_item) => {
                    animal_validated.set(animal_validated_item);
                    open.set(true);
                }
                Err(error) => {
                    animal_value.set((&error, &animal).into());
                    animal_error.set(Some(error));
                }
            }
        }
    };

    let submit = move |_| async move {
        let validated_animal = animal_validated.cloned();
        animal_value.reset();
        animal_error.reset();
        add_animal(validated_animal.into())
            .await
            .unwrap_or_else(|_| {
                navigator().push(Route::ErrorPage {});
            });
        animals.restart();
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
        form { class: "form", onsubmit: alert,
            AnimalFormBody { animal_value: animal_value_clone, animal_input: animal_input,
                animal_validation_error: animal_error_clone }
            button { class: "btn btn-skyblue", type: "submit", "Add"}
        }
        AlertDialogRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            class: "alert-dialog-backdrop",
            AlertDialogContent { class: "alert-dialog",
                AlertDialogTitle { "Add Animal" }
                AlertDialogDescription { "Are you sure you want to add this animal?" }
                AlertDialogActions {
                    class: "alert-dialog-actions",
                    AlertDialogCancel { class: "alert-dialog-cancel mr-1", "Cancel" }
                    AlertDialogAction {
                        class: "alert-dialog-action",
                        on_click: submit,
                        "Add Animal"
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditAnimal(id: i64) -> Element {
    let animal = use_resource(move || async move {
        fetch_animal_by_id(id).await.unwrap_or_else(|_| {
            navigator().push(Route::ErrorPage {});
            AnimalModel::default()
        })
    });
    let mut animal_input = use_signal(|| AnimalModel::default());
    let mut animal_value = use_signal(|| AnimalModel::default());
    let mut animal_validated = use_signal(|| AnimalValidated::default());
    let mut animal_error = use_signal(|| Option::<AnimalValidationError>::None);
    let mut open = use_signal(|| false);

    let animal_error_clone = animal_error.cloned();

    if animal_error_clone.is_none() {
        animal_input.set(animal.cloned().unwrap_or_default());
        animal_value.set(animal.cloned().unwrap_or_default());
    }

    let alert = move |e: Event<FormData>| {
        e.prevent_default();
        async move {
            let animal = animal_input.cloned();
            match animal.validate() {
                Ok(animal_validated_item) => {
                    animal_validated.set(animal_validated_item);
                    open.set(true);
                }
                Err(error) => {
                    animal_value.set((&error, &animal).into());
                    animal_error.set(Some(error));
                }
            }
        }
    };

    let submit = move |_| async move {
        let validated_animal = animal_validated.cloned();
        animal_error.reset();
        edit_animal(id, validated_animal.into())
            .await
            .unwrap_or_else(|_| {
                navigator().push(Route::ErrorPage {});
            });
        navigator().push(Route::Animal {});
    };

    let animal_error_clone = animal_error_clone.unwrap_or_default();
    let animal_value_clone = animal_value.cloned();

    rsx! {
        Title { "Edit Animal" }
        h1 { "Edit Animal" }
        form { class: "form", onsubmit: alert,
            AnimalFormBody { animal_value: animal_value_clone, animal_input: animal_input,
                animal_validation_error: animal_error_clone }
            button { class: "btn btn-skyblue", type: "submit", "Edit"}
        }
        Link { class: "btn btn-skyblue inline-block", to: Route::Animal { }, "Back to Animal" }
        AlertDialogRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            class: "alert-dialog-backdrop",
            AlertDialogContent { class: "alert-dialog",
                AlertDialogTitle { "Edit Animal" }
                AlertDialogDescription { "Are you sure you want to edit this animal?" }
                AlertDialogActions {
                    class: "alert-dialog-actions",
                    AlertDialogCancel { class: "alert-dialog-cancel mr-1", "Cancel" }
                    AlertDialogAction {
                        class: "alert-dialog-action",
                        on_click: submit,
                        "Edit Animal"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ErrorMessage(msgs: ValidateErrorStore) -> Element {
    let i18n = i18n();
    let msgs = msgs.as_translated_message(&i18n);
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
