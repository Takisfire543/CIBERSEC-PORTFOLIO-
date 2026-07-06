use dioxus::prelude::*;

pub const GLOBAL_STYLE: &str = r#"
/* ===========================
   VARIABLES Y COLORES
   =========================== */
:root {
    --primary: #00ff41;
    --primary-dark: #00cc33;
    --primary-light: #33ff66;
    --dark: #0a0e27;
    --darker: #050a17;
    --accent: #ff006e;
    --accent-light: #ff3399;
    --text: #e0e0e0;
    --text-muted: #888;
    --border: #1a1f3a;
    --bg-hover: rgba(0, 255, 65, 0.1);
}

/* ===========================
   RESET Y BASE
   =========================== */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

html {
    scroll-behavior: smooth;
}

body {
    font-family: 'Courier New', 'Source Code Pro', monospace;
    background: linear-gradient(135deg, var(--darker) 0%, var(--dark) 50%, #0f1436 100%);
    color: var(--text);
    line-height: 1.6;
    overflow-x: hidden;
}

.app {
    width: 100%;
    min-height: 100vh;
}

/* ===========================
   HEADER / NAVBAR
   =========================== */
.header {
    position: sticky;
    top: 0;
    z-index: 1000;
    background: rgba(5, 10, 23, 0.98);
    backdrop-filter: blur(15px);
    border-bottom: 2px solid var(--border);
    box-shadow: 0 4px 20px rgba(0, 255, 65, 0.1);
    padding: 1.2rem 2rem;
}

.header-container {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.logo {
    font-size: 1.8rem;
    font-weight: 900;
    color: var(--primary);
    text-shadow: 0 0 15px var(--primary), 0 0 30px rgba(0, 255, 65, 0.3);
    letter-spacing: 2px;
    transition: all 0.3s ease;
}

.logo:hover {
    text-shadow: 0 0 25px var(--primary), 0 0 50px rgba(0, 255, 65, 0.5);
    transform: scale(1.05);
}

.nav-links {
    display: flex;
    gap: 2.5rem;
    list-style: none;
}

.nav-links a {
    color: var(--text);
    text-decoration: none;
    transition: all 0.3s ease;
    font-weight: 600;
    position: relative;
    padding: 0.5rem 0;
}

.nav-links a::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    width: 0;
    height: 2px;
    background: linear-gradient(90deg, var(--primary), var(--accent));
    transition: width 0.3s ease;
}

.nav-links a:hover {
    color: var(--primary);
    text-shadow: 0 0 10px var(--primary);
}

.nav-links a:hover::after {
    width: 100%;
}

/* ===========================
   HERO SECTION
   =========================== */
.hero {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 4rem 2rem;
    background: linear-gradient(180deg, rgba(0, 255, 65, 0.08) 0%, rgba(255, 0, 110, 0.05) 50%, transparent 100%),
                radial-gradient(circle at 20% 50%, rgba(0, 255, 65, 0.1) 0%, transparent 50%);
    position: relative;
    overflow: hidden;
}

.hero::before {
    content: '';
    position: absolute;
    top: -50%;
    right: -50%;
    width: 100%;
    height: 100%;
    background: radial-gradient(circle, rgba(0, 255, 65, 0.03) 1px, transparent 1px);
    background-size: 50px 50px;
    animation: float 20s infinite linear;
}

@keyframes float {
    0% { transform: translate(0, 0); }
    100% { transform: translate(50px, 50px); }
}

.hero-content {
    position: relative;
    z-index: 10;
    max-width: 900px;
}

.hero-content h1 {
    font-size: 4.5rem;
    margin-bottom: 1.5rem;
    color: var(--primary);
    text-shadow: 0 0 30px var(--primary), 0 0 60px rgba(0, 255, 65, 0.4);
    letter-spacing: 3px;
    animation: glow-pulse 3s ease-in-out infinite;
    font-weight: 900;
}

.hero-content p {
    font-size: 1.5rem;
    color: var(--text-muted);
    margin-bottom: 0.8rem;
    font-weight: 300;
    animation: fadeInUp 0.8s ease-out forwards;
}

.hero-content p:nth-of-type(2) {
    animation-delay: 0.2s;
    color: var(--primary-light);
    font-weight: 600;
}

.hero-content p:nth-of-type(3) {
    animation-delay: 0.4s;
}

.cta-button {
    display: inline-block;
    margin-top: 2rem;
    padding: 1rem 2.5rem;
    background: linear-gradient(135deg, var(--primary) 0%, var(--primary-light) 100%);
    color: var(--darker);
    border: 2px solid var(--primary);
    border-radius: 5px;
    cursor: pointer;
    font-weight: bold;
    font-size: 1.1rem;
    transition: all 0.3s ease;
    text-decoration: none;
    font-family: 'Courier New', monospace;
    letter-spacing: 1px;
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.3);
    animation: fadeInUp 0.8s ease-out 0.6s forwards;
    opacity: 0;
}

.cta-button:hover {
    transform: translateY(-3px) scale(1.05);
    box-shadow: 0 0 40px rgba(0, 255, 65, 0.6), 0 10px 30px rgba(0, 255, 65, 0.2);
    background: var(--primary);
}

@keyframes glow-pulse {
    0%, 100% {
        text-shadow: 0 0 30px var(--primary), 0 0 60px rgba(0, 255, 65, 0.4);
    }
    50% {
        text-shadow: 0 0 50px var(--primary), 0 0 100px rgba(0, 255, 65, 0.6);
    }
}

@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* ===========================
   ABOUT SECTION
   =========================== */
.about {
    padding: 6rem 2rem;
    background: rgba(0, 0, 0, 0.4);
    border-top: 2px solid var(--border);
    border-bottom: 2px solid var(--border);
}

.section-container {
    max-width: 1400px;
    margin: 0 auto;
}

.section-title {
    font-size: 3rem;
    margin-bottom: 3rem;
    color: var(--primary);
    text-align: center;
    text-shadow: 0 0 20px var(--primary), 0 0 40px rgba(0, 255, 65, 0.3);
    letter-spacing: 2px;
    position: relative;
    font-weight: 900;
}

.section-title::after {
    content: '';
    display: block;
    width: 100px;
    height: 3px;
    background: linear-gradient(90deg, var(--primary), var(--accent));
    margin: 1rem auto 0;
    border-radius: 2px;
    box-shadow: 0 0 15px var(--primary);
}

.about-content {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4rem;
    align-items: center;
}

.about-text {
    font-size: 1.1rem;
    line-height: 2;
    color: var(--text);
}

.about-text p {
    margin-bottom: 1.5rem;
    padding-left: 2rem;
    border-left: 3px solid var(--primary);
    transition: all 0.3s ease;
}

.about-text p:hover {
    border-left-color: var(--accent);
    color: var(--primary-light);
}

.about-text h3 {
    color: var(--primary);
    font-size: 1.5rem;
    margin-bottom: 1.5rem;
    text-shadow: 0 0 10px var(--primary);
}

.about-text ul {
    list-style: none;
    font-size: 1.1rem;
}

.about-text li {
    padding: 0.6rem 0;
    transition: all 0.3s ease;
    cursor: default;
}

.about-text li:hover {
    transform: translateX(10px);
    color: var(--primary);
    text-shadow: 0 0 10px var(--primary);
}

/* ===========================
   SKILLS SECTION
   =========================== */
.skills {
    padding: 6rem 2rem;
    background: linear-gradient(180deg, rgba(0, 255, 65, 0.03) 0%, transparent 100%);
}

.skills-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 2.5rem;
    margin-top: 3rem;
}

.skill-card {
    background: linear-gradient(135deg, rgba(0, 255, 65, 0.08) 0%, rgba(0, 255, 65, 0.02) 100%);
    border: 2px solid var(--border);
    padding: 2rem;
    border-radius: 8px;
    transition: all 0.3s ease;
    cursor: pointer;
    position: relative;
    overflow: hidden;
}

.skill-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(0, 255, 65, 0.2), transparent);
    transition: left 0.5s ease;
}

.skill-card:hover::before {
    left: 100%;
}

.skill-card:hover {
    background: linear-gradient(135deg, rgba(0, 255, 65, 0.15) 0%, rgba(255, 0, 110, 0.05) 100%);
    border-color: var(--primary);
    box-shadow: 0 0 25px rgba(0, 255, 65, 0.3), inset 0 0 20px rgba(0, 255, 65, 0.05);
    transform: translateY(-10px);
    border-radius: 12px;
}

.skill-card h3 {
    color: var(--primary);
    margin-bottom: 1.5rem;
    font-size: 1.4rem;
    text-shadow: 0 0 10px var(--primary);
    letter-spacing: 1px;
}

.skill-card ul {
    list-style: none;
}

.skill-card li {
    padding: 0.7rem 0;
    color: var(--text-muted);
    transition: all 0.3s ease;
    font-size: 1rem;
}

.skill-card li:before {
    content: "▸ ";
    color: var(--accent);
    margin-right: 0.8rem;
    font-weight: bold;
    transition: all 0.3s ease;
}

.skill-card:hover li {
    color: var(--text);
    padding-left: 0.5rem;
}

/* ===========================
   PROJECTS SECTION
   =========================== */
.projects {
    padding: 6rem 2rem;
    background: rgba(0, 0, 0, 0.3);
    border-top: 2px solid var(--border);
}

.projects-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 3rem;
    margin-top: 3rem;
}

.project-card {
    background: linear-gradient(135deg, rgba(255, 0, 110, 0.08) 0%, rgba(0, 255, 65, 0.02) 100%);
    border: 2px solid var(--border);
    padding: 2.5rem;
    border-radius: 8px;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
}

.project-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background: linear-gradient(90deg, var(--primary), var(--accent));
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 0.3s ease;
}

.project-card:hover::before {
    transform: scaleX(1);
}

.project-card:hover {
    border-color: var(--accent);
    background: linear-gradient(135deg, rgba(255, 0, 110, 0.12) 0%, rgba(0, 255, 65, 0.05) 100%);
    box-shadow: 0 0 30px rgba(255, 0, 110, 0.2), inset 0 0 20px rgba(0, 255, 65, 0.03);
    transform: translateY(-12px);
    border-radius: 12px;
}

.project-card h3 {
    color: var(--accent);
    margin-bottom: 0.8rem;
    font-size: 1.4rem;
    text-shadow: 0 0 10px var(--accent);
}

.project-card .date {
    font-size: 0.95rem;
    color: var(--primary);
    margin-bottom: 1.2rem;
    font-weight: 600;
}

.project-card p {
    color: var(--text);
    line-height: 1.8;
    margin-bottom: 1.5rem;
    font-size: 1rem;
}

.project-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.8rem;
}

.tag {
    display: inline-block;
    background: rgba(0, 255, 65, 0.1);
    color: var(--primary);
    padding: 0.5rem 1rem;
    border-radius: 25px;
    font-size: 0.9rem;
    border: 1px solid var(--primary);
    transition: all 0.3s ease;
    cursor: default;
}

.tag:hover {
    background: var(--primary);
    color: var(--darker);
    box-shadow: 0 0 15px var(--primary);
    transform: scale(1.1);
}

/* ===========================
   CONTACT SECTION
   =========================== */
.contact {
    padding: 6rem 2rem;
    text-align: center;
    background: linear-gradient(180deg, transparent 0%, rgba(0, 255, 65, 0.05) 100%);
    border-top: 2px solid var(--border);
}

.contact-content {
    max-width: 800px;
    margin: 0 auto;
}

.contact-content p {
    font-size: 1.2rem;
    margin-bottom: 1rem;
    color: var(--text);
}

.contact-links {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    margin-top: 2.5rem;
    flex-wrap: wrap;
}

.contact-link {
    color: var(--text);
    text-decoration: none;
    padding: 1rem 2rem;
    border: 2px solid var(--border);
    border-radius: 8px;
    transition: all 0.3s ease;
    font-weight: 600;
    font-size: 1.05rem;
    position: relative;
    overflow: hidden;
}

.contact-link::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: var(--primary);
    z-index: -1;
    transition: left 0.3s ease;
}

.contact-link:hover::before {
    left: 0;
}

.contact-link:hover {
    color: var(--darker);
    border-color: var(--primary);
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.5);
    transform: translateY(-3px);
}

/* ===========================
   FOOTER
   =========================== */
.footer {
    text-align: center;
    padding: 2.5rem 2rem;
    background: rgba(0, 0, 0, 0.6);
    border-top: 2px solid var(--border);
    color: var(--text-muted);
    font-size: 0.95rem;
}

.footer p {
    margin: 0.5rem 0;
}

.footer p:first-child {
    color: var(--primary);
    font-weight: 600;
}

/* ===========================
   RESPONSIVE DESIGN
   =========================== */
@media (max-width: 1024px) {
    .hero-content h1 {
        font-size: 3.5rem;
    }

    .about-content {
        grid-template-columns: 1fr;
        gap: 2rem;
    }

    .section-title {
        font-size: 2.5rem;
    }
}

@media (max-width: 768px) {
    .nav-links {
        gap: 1rem;
        font-size: 0.9rem;
    }

    .hero {
        min-height: 80vh;
        padding: 2rem 1rem;
    }

    .hero-content h1 {
        font-size: 2.5rem;
        letter-spacing: 1px;
    }

    .hero-content p {
        font-size: 1.1rem;
    }

    .section-title {
        font-size: 2rem;
    }

    .section-container {
        padding: 0 1rem;
    }

    .about, .skills, .projects, .contact {
        padding: 3rem 1rem;
    }

    .about-text {
        font-size: 1rem;
    }

    .skills-grid,
    .projects-grid {
        grid-template-columns: 1fr;
        gap: 1.5rem;
    }

    .contact-links {
        flex-direction: column;
        gap: 1rem;
    }

    .contact-link {
        width: 100%;
    }

    .logo {
        font-size: 1.3rem;
        letter-spacing: 1px;
    }
}

@media (max-width: 480px) {
    .hero-content h1 {
        font-size: 1.8rem;
    }

    .hero-content p {
        font-size: 0.95rem;
    }

    .section-title {
        font-size: 1.6rem;
    }

    .skill-card,
    .project-card {
        padding: 1.5rem;
    }

    .nav-links {
        gap: 0.7rem;
        font-size: 0.8rem;
    }

    .cta-button {
        padding: 0.8rem 1.5rem;
        font-size: 0.95rem;
    }
}

/* ===========================
   SCROLLBAR PERSONALIZADO
   =========================== */
::-webkit-scrollbar {
    width: 12px;
}

::-webkit-scrollbar-track {
    background: var(--darker);
}

::-webkit-scrollbar-thumb {
    background: var(--primary);
    border-radius: 6px;
    box-shadow: 0 0 10px var(--primary);
}

::-webkit-scrollbar-thumb:hover {
    background: var(--primary-light);
}
"#;
