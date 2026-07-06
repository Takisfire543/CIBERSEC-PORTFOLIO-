use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                h1 { "EDMUNDO RAMIRZ" }
                p { "Especialista en Ciberseguridad & Ethical Hacker" }
                p { "Protegiendo sistemas, descubriendo vulnerabilidades" }
                button { class: "cta-button",
                    "Descargar CV"
                }
            }
        }
    }
}
