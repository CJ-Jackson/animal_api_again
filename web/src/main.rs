use dioxus::prelude::*;

use ui::Animal;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        Animal {}
    }
}
