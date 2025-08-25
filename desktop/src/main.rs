use dioxus::prelude::*;

use ui::UiApp;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        UiApp {}
    }
}
