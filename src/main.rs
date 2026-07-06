use dioxus::prelude::*;

mod components;
mod styles;

use components::{Header, Hero, About, Skills, Projects, Contact, Footer};
use styles::GLOBAL_STYLE;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {GLOBAL_STYLE} }
        div { class: "app",
            Header {}
            Hero {}
            About {}
            Skills {}
            Projects {}
            Contact {}
            Footer {}
        }
    }
}
