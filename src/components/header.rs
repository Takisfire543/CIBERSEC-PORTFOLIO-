use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "header-container",
                div { class: "logo",
                    "[ EDMUNDO RAMIRZ ]"
                }
                nav {
                    ul { class: "nav-links",
                        li {
                            a { href: "#", "Inicio" }
                        }
                        li {
                            a { href: "#about", "Sobre Mí" }
                        }
                        li {
                            a { href: "#skills", "Habilidades" }
                        }
                        li {
                            a { href: "#projects", "Proyectos" }
                        }
                        li {
                            a { href: "#contact", "Contacto" }
                        }
                    }
                }
            }
        }
    }
}
