use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { class: "about", id: "about",
            div { class: "section-container",
                h2 { class: "section-title", "Sobre Mí" }
                div { class: "about-content",
                    div { class: "about-text",
                        p {
                            "Soy un especialista en ciberseguridad apasionado por proteger sistemas y datos críticos. Con experiencia en pentesting, análisis de vulnerabilidades y seguridad ofensiva."
                        }
                        p {
                            "Mi enfoque combina metodologías de seguridad defensiva y ofensiva para identificar y mitigar riesgos antes de que se conviertan en amenazas reales."
                        }
                        p {
                            "Actualmente trabajo en el fortalecimiento de infraestructuras de seguridad, respuesta ante incidentes y conciencia de seguridad en organizaciones."
                        }
                    }
                    div { class: "about-text",
                        h3 { style: "color: var(--primary); margin-bottom: 1rem;",
                            "Especialidades:"
                        }
                        ul { style: "list-style: none; line-height: 2;",
                            li { "✓ Pentesting & Ethical Hacking" }
                            li { "✓ Análisis de Vulnerabilidades" }
                            li { "✓ Seguridad en la Nube" }
                            li { "✓ Forensica Digital" }
                            li { "✓ Respuesta a Incidentes" }
                            li { "✓ Hardening de Sistemas" }
                        }
                    }
                }
            }
        }
    }
}
