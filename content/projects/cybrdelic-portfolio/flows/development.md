# Development Guide

This guide covers the development workflow and patterns used in the Cybrdelic Portfolio project. Follow these guidelines to maintain consistency when making changes or adding features.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.65 or newer
- [Node.js](https://nodejs.org/) 16 or newer (for frontend tooling)
- [Git](https://git-scm.com/downloads)

### Local Development Setup

1. Clone the repository:

```bash
git clone https://github.com/cybrdelic/cybrdelic-portfolio.git
cd cybrdelic-portfolio
```

2. Install dependencies:

```bash
cargo build
npm install
```

3. Start the development server:

```bash
cargo run
```

This will start the server at http://localhost:3000.

## Project Structure

The project follows a modular organization:

```
cybrdelic-portfolio/
├── content/               # Markdown content files
├── src/                   # Rust source code
├── static/                # Static assets
├── templates/             # HTML templates
└── Cargo.toml             # Rust package manifest
```

## Development Workflow

### Adding a New Page

1. Create a new handler in `src/handlers/`:

```rust
// src/handlers/new_page.rs
use crate::{AppError, AppState};
use axum::{extract::State, response::Html};
use tera::Context;

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let mut ctx = Context::new();
    
    // Add data to context
    ctx.insert("page_title", "New Page");
    
    match state.tera.render("new_page.html", &ctx) {
        Ok(html) => Ok(Html(html)),
        Err(err) => Err(AppError::Template(err)),
    }
}
```

2. Register the route in `src/main.rs`:

```rust
// Add to existing router
let app = Router::new()
    // Existing routes...
    .route("/new-page", get(handlers::new_page::index))
    // ...
```

3. Create a template in `templates/new_page.html`:

```html
{% extends "base.html" %}

{% block content %}
<main class="new-page-container">
    <h1>New Page</h1>
    <!-- Page content here -->
</main>
{% endblock %}
```

4. Add CSS in `static/css/new_page.css`:

```css
/* New page styles */
.new-page-container {
    /* styles here */
}
```

5. Import the CSS in `static/css/main.css`:

```css
@import 'new_page.css';
```

### Adding a New Component

1. Create a new file in `templates/components/`:

```html
{# components/new_component.html #}
<div class="new-component">
    <h2>{{ title }}</h2>
    <p>{{ description }}</p>
</div>
```

2. Add component styles in `static/css/components/new_component.css`:

```css
.new-component {
    /* Component styles */
}
```

3. Import the component styles in `static/css/main.css`:

```css
@import 'components/new_component.css';
```

4. Use the component in a template:

```html
{% include "components/new_component.html" with title="Example" description="This is an example" %}
```

## Coding Standards

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` to catch common mistakes
- Document public functions and types with doc comments

### HTML

- Use semantic HTML5 elements
- Follow a component-based approach
- Keep templates modular and reusable
- Use Tera's inheritance and includes

### CSS

- Follow BEM (Block Element Modifier) naming convention
- Organize CSS into logical modules
- Use CSS variables for theming and consistency
- Prefix utility classes appropriately

### JavaScript

- Write modular, self-contained functions
- Use ES6+ features but ensure browser compatibility
- Add appropriate comments for complex logic
- Follow the principle of progressive enhancement

## Testing

### Rust Tests

Run Rust tests with:

```bash
cargo test
```

### Frontend Tests

For CSS and JavaScript:

1. Visual testing against design specifications
2. Cross-browser testing (Chrome, Firefox, Safari)
3. Responsive design testing

## Deployment

The portfolio is deployed using a simple build process:

1. Build the Rust application:

```bash
cargo build --release
```

2. Optimize static assets:

```bash
npm run build
```

3. Deploy the built application:

```bash
./deploy.sh
```

## Common Tasks

### Adding a New Project

1. Create a new directory in `content/projects/`:

```bash
mkdir -p content/projects/new-project/flows
```

2. Add project markdown files:

```bash
touch content/projects/new-project/flows/architecture.md
```

3. Update the project list in `src/handlers/projects.rs`

### Updating the Theme

1. Modify theme variables in `static/css/variables/variables.css`
2. Test in both light and dark modes