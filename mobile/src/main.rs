use dioxus::prelude::*;

use ui::Animal;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Animal {}
    }
}
