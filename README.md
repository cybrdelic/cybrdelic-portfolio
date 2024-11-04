# Cybrdelic Portfolio

A cyberpunk-themed personal portfolio showcasing systems architecture and AI development work, built with Rust (Axum) and HTMX. Features a terminal-inspired UI with dynamic content loading and cyberpunk aesthetics.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Axum Version](https://img.shields.io/badge/axum-0.7-blue.svg)
![HTMX Version](https://img.shields.io/badge/htmx-1.9.10-green.svg)

## 🚀 Features

- **Terminal-Inspired UI**: Modern interpretation of terminal aesthetics with cyberpunk styling
- **Dynamic Content**: HTMX integration for smooth, server-side rendered content updates
- **Ambient Effects**: Dynamic grid overlay, scan lines, and glitch effects
- **Responsive Design**: Optimized layout across all device sizes
- **Performance Focused**:
  - Resource preloading
  - Critical CSS inlining
  - Progressive font loading
  - Optimized animations
- **Accessibility**: Semantic HTML, ARIA labels, and keyboard navigation support

## 🛠 Tech Stack

### Backend
- **Rust + Axum**: Modern web framework for efficient server-side operations
- **Tokio**: Asynchronous runtime for handling concurrent operations
- **Tower-HTTP**: Middleware stack for static file serving and request tracing

### Frontend
- **HTMX**: Dynamic content loading without complex JavaScript
- **Vanilla JavaScript**: Custom animations and UI interactions
- **Modern CSS**: Custom properties, animations, and responsive design
- **Fonts**: JetBrains Mono, Space Grotesk, and Share Tech Mono

## 📦 Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/cybrdelic-portfolio.git
cd cybrdelic-portfolio
```

2. **Build and run**
```bash
cargo build
cargo run
```

The server will start at `http://localhost:3000`

## 📂 Project Structure

```
.
├── Cargo.toml
├── src/
│   ├── handlers/
│   │   ├── contact.rs
│   │   ├── mod.rs
│   │   ├── projects.rs
│   │   └── static_files.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── project.rs
│   ├── routes/
│   │   └── mod.rs
│   ├── setup/
│   │   ├── app_config.rs
│   │   └── mod.rs
│   └── main.rs
└── static/
    ├── components/
    │   ├── about/
    │   ├── contact/
    │   ├── header/
    │   ├── hero/
    │   └── projects/
    ├── index.html
    ├── js/
    │   └── main.js
    ├── project-template.html
    └── styles/
        ├── components/
        └── main.css
```

## 🎨 Theme Customization

The project uses CSS custom properties for theming. Main colors and variables can be modified in `static/styles/main.css`:

```css
:root {
  --color-bg: #0a0a0b;
  --color-text: #e2e2e4;
  --color-primary: #00ff9f;
  --color-secondary: #4258ff;
  --color-accent: #ff2b51;
  /* Additional variables available for customization */
}
```

## 🔧 Development

### Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Server Configuration
The server is configured to run on `0.0.0.0:3000` by default. This can be modified in `src/main.rs`.

### Static Files
Static files are served from the `static/` directory, including:
- HTML templates
- JavaScript files
- CSS stylesheets
- Component templates

### HTMX Integration
The project uses HTMX 1.9.10 for dynamic content loading. Main integration points:
- Project grid loading
- Contact form submission
- Dynamic content updates

## 📱 Responsive Design

The portfolio is fully responsive with breakpoints at:
- 1600px: Large desktop optimization
- 1200px: Desktop/tablet transition
- 768px: Tablet/mobile transition
- 480px: Small mobile optimization

## 🚀 Performance Features

- **Resource Preloading**:
  ```html
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preload" href="/static/js/main.js" as="script">
  <link rel="preload" href="/static/styles/main.css" as="style">
  ```

- **Critical CSS** inlined in `index.html`
- **Progressive Font Loading**
- **Optimized Animations** with reduced motion support
- **Efficient Asset Loading** with deferred scripts

## 📄 License

This project is licensed under the MIT License.

---

For bug reports and feature requests, please open an issue on GitHub.
