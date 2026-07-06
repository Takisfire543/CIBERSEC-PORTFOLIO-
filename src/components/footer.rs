use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { "© 2024 Edmundo Ramirz. Todos los derechos reservados." }
            p { "Ciberseguridad | Ethical Hacking | Protección de Datos" }
        }
    }
}
