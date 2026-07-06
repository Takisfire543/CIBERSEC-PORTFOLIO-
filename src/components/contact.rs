use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { class: "contact", id: "contact",
            div { class: "section-container",
                h2 { class: "section-title", "Contacto" }
                div { class: "contact-content",
                    p { "¿Interesado en colaborar o discutir proyectos de seguridad?" }
                    p { "Contáctame a través de los siguientes canales:" }
                    div { class: "contact-links",
                        a { class: "contact-link", href: "mailto:edmundo@email.com", "📧 Email" }
                        a { class: "contact-link", href: "https://linkedin.com", target: "_blank", "💼 LinkedIn" }
                        a { class: "contact-link", href: "https://github.com", target: "_blank", "🔐 GitHub" }
                        a { class: "contact-link", href: "https://twitter.com", target: "_blank", "🐦 Twitter" }
                    }
                }
            }
        }
    }
}
