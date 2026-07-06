use dioxus::prelude::*;

#[component]
pub fn Skills() -> Element {
    rsx! {
        section { class: "skills", id: "skills",
            div { class: "section-container",
                h2 { class: "section-title", "Habilidades" }
                div { class: "skills-grid",
                    SkillCard {
                        title: "Herramientas de Seguridad",
                        skills: vec![
                            "Burp Suite",
                            "Metasploit",
                            "Wireshark",
                            "Nmap",
                            "Hashcat"
                        ]
                    }
                    SkillCard {
                        title: "Lenguajes",
                        skills: vec![
                            "Python",
                            "Bash",
                            "JavaScript",
                            "Rust",
                            "PowerShell"
                        ]
                    }
                    SkillCard {
                        title: "Seguridad de Redes",
                        skills: vec![
                            "Análisis de Tráfico",
                            "IDS/IPS",
                            "Firewalls",
                            "VPN",
                            "Zero Trust"
                        ]
                    }
                    SkillCard {
                        title: "Cloud Security",
                        skills: vec![
                            "AWS Security",
                            "Azure Security",
                            "IAM",
                            "Compliance",
                            "SIEM"
                        ]
                    }
                }
            }
        }
    }
}

#[component]
fn SkillCard(title: String, skills: Vec<&'static str>) -> Element {
    rsx! {
        div { class: "skill-card",
            h3 { "{title}" }
            ul {
                {skills.iter().map(|skill| rsx! {
                    li { key: "{skill}", "{skill}" }
                })}
            }
        }
    }
}
