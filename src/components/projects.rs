use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { class: "projects", id: "projects",
            div { class: "section-container",
                h2 { class: "section-title", "Proyectos Destacados" }
                div { class: "projects-grid",
                    ProjectCard {
                        title: "Scanner de Vulnerabilidades Avanzado",
                        date: "2024",
                        description: "Herramienta automatizada para escaneo de vulnerabilidades en aplicaciones web y servidores.",
                        tags: vec!["Python", "Seguridad", "Automatización"]
                    }
                    ProjectCard {
                        title: "Sistema de Respuesta a Incidentes",
                        date: "2023",
                        description: "Plataforma integrada para análisis y respuesta rápida ante incidentes de seguridad.",
                        tags: vec!["SIEM", "Análisis", "Respuesta"]
                    }
                    ProjectCard {
                        title: "Análisis Forense Digital",
                        date: "2023",
                        description: "Suite de herramientas para investigación y análisis forense de dispositivos y sistemas.",
                        tags: vec!["Forensica", "Linux", "Investigación"]
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(title: String, date: &'static str, description: String, tags: Vec<&'static str>) -> Element {
    rsx! {
        div { class: "project-card",
            h3 { "{title}" }
            p { class: "date", "{date}" }
            p { "{description}" }
            div { class: "project-tags",
                {tags.iter().map(|tag| rsx! {
                    span { key: "{tag}", class: "tag", "{tag}" }
                })}
            }
        }
    }
}
