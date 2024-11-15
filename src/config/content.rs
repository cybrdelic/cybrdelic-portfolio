use serde::Serialize;
use std::collections::HashMap;
use slug::slugify;

#[derive(Serialize, Clone)]
pub struct SiteContent {
    pub meta: MetaContent,
    pub navigation: NavigationContent,
    pub hero: HeroContent,
    pub projects: ProjectsContent,
    pub skills: SkillsContent,
    pub contact: ContactContent,
}

#[derive(Serialize, Clone)]
pub struct MetaContent {
    pub site_title: &'static str,
    pub author: &'static str,
    pub description: &'static str,
}

#[derive(Serialize, Clone)]
pub struct NavigationContent {
    pub home_text: &'static str,
    pub projects_text: &'static str,
    pub exploits_text: &'static str,
    pub contact_text: &'static str,
    pub github_text: &'static str,
    pub github_url: &'static str,
}

#[derive(Serialize, Clone)]
pub struct HeroContent {
    pub ascii_art: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub description: &'static str,
}

#[derive(Serialize, Clone)]
pub struct ProjectDetail {
    pub name: &'static str,
    pub slug: String,
    pub tags: Vec<&'static str>,
    pub description: &'static str,
    pub long_description: &'static str,
    pub source_url: &'static str,
    pub live_url: Option<&'static str>,
    pub tech_stack: Vec<&'static str>,
    pub features: Vec<&'static str>,
    pub gallery: Vec<&'static str>,
    pub challenges: Vec<&'static str>,
    pub next_steps: Vec<&'static str>,
}

impl ProjectDetail {
    pub fn new(
        name: &'static str,
        tags: Vec<&'static str>,
        description: &'static str,
        long_description: &'static str,
        source_url: &'static str,
        live_url: Option<&'static str>,
        tech_stack: Vec<&'static str>,
        features: Vec<&'static str>,
        gallery: Vec<&'static str>,
        challenges: Vec<&'static str>,
        next_steps: Vec<&'static str>,
    ) -> Self {
        ProjectDetail {
            name,
            slug: slugify(name),
            tags,
            description,
            long_description,
            source_url,
            live_url,
            tech_stack,
            features,
            gallery,
            challenges,
            next_steps,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ProjectsContent {
    pub title: &'static str,
    pub projects: Vec<ProjectDetail>,
}

#[derive(Serialize, Clone)]
pub struct SkillCategory {
    pub title: &'static str,
    pub skills: Vec<&'static str>,
}

#[derive(Serialize, Clone)]
pub struct SkillsContent {
    pub title: &'static str,
    pub categories: Vec<SkillCategory>,
}

#[derive(Serialize, Clone)]
pub struct ContactContent {
    pub title: &'static str,
    pub username_label: &'static str,
    pub email_label: &'static str,
    pub message_label: &'static str,
    pub submit_text: &'static str,
}

impl Default for SiteContent {
    fn default() -> Self {
        let projects = vec![
            ProjectDetail::new(
                "kernel_exploits",
                vec!["rust", "kernel", "security"],
                "Custom Linux kernel module for security research. Implements syscall hooking and memory manipulation techniques.",
                r#"A comprehensive Linux kernel module designed for security research and analysis. This project demonstrates advanced systems programming concepts and security principles through practical implementation.

Key aspects of the project include:
- Custom syscall table manipulation
- Memory management subsystem integration
- Security hook implementation
- Real-time system monitoring capabilities"#,
                "https://github.com/yourusername/kernel_exploits",
                None,
                vec!["Rust", "Linux Kernel", "C", "Assembly", "LLVM"],
                vec![
                    "Syscall hooking engine",
                    "Memory manipulation framework",
                    "Security policy enforcement",
                    "Runtime integrity verification",
                ],
                vec!["arch.svg", "flow.svg", "demo.svg"],
                vec![
                    "Kernel API compatibility across versions",
                    "Race condition mitigation",
                    "Security boundary preservation",
                ],
                vec![
                    "eBPF integration",
                    "Extended syscall coverage",
                    "Real-time monitoring dashboard",
                ]
            ),
            ProjectDetail::new(
                "quantum_noise",
                vec!["rust", "webgl", "art"],
                "Real-time WebGL noise visualization using quantum randomness. GPU-accelerated shader effects.",
                r#"An experimental art project that combines quantum mechanics principles with real-time graphics processing to create unique visual experiences.

The system generates true random noise patterns based on quantum phenomena and transforms them into mesmerizing visual displays through custom WebGL shaders."#,
                "https://github.com/yourusername/quantum_noise",
                Some("https://quantum-noise.demo"),
                vec!["Rust", "WebGL", "GLSL", "WebAssembly"],
                vec![
                    "Quantum random number generation",
                    "Custom shader pipeline",
                    "Real-time visualization",
                    "Interactive parameters",
                ],
                vec!["preview.svg", "interface.svg", "patterns.svg"],
                vec![
                    "WebGL performance optimization",
                    "Quantum data integration",
                    "Cross-platform compatibility",
                ],
                vec![
                    "Audio reactivity",
                    "Machine learning integration",
                    "VR support",
                ]
            ),
            // Add more projects here in the same format
        ];

        SiteContent {
            meta: MetaContent {
                site_title: "Cybrdelic",
                author: "Cybrdelic Dev",
                description: "Security Research & Creative Code",
            },
            navigation: NavigationContent {
                home_text: "cd /home",
                projects_text: "ls projects/",
                exploits_text: "cat exploits.txt",
                contact_text: "./contact",
                github_text: "git clone",
                github_url: "https://github.com/yourusername",
            },
            hero: HeroContent {
                ascii_art: r#" [ASCII art here] "#,
                title: "Cybrdelic Dev :: Security Researcher | Rust Developer | Digital Artist",
                subtitle: "$ whoami",
                description: "Exploring the intersection of systems programming, security, and digital art",
            },
            projects: ProjectsContent {
                title: "$ ls -la ~/projects",
                projects,
            },
            skills: SkillsContent {
                title: "$ cat exploits.txt",
                categories: vec![
                    SkillCategory {
                        title: "Systems Programming",
                        skills: vec![
                            "Kernel module development",
                            "Memory management",
                            "System calls",
                        ],
                    },
                    // Add more categories
                ],
            },
            contact: ContactContent {
                title: "$ ./contact --init",
                username_label: "username:",
                email_label: "email:",
                message_label: "message:",
                submit_text: "./send.sh",
            },
        }
    }
}

pub fn get_content() -> SiteContent {
    SiteContent::default()
}

pub fn get_project_by_slug(slug: &str) -> Option<ProjectDetail> {
    get_content()
        .projects
        .projects
        .into_iter()
        .find(|p| p.slug == slug)
}
