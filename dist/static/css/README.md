# Cybrdelic Portfolio - CSS Architecture

This document outlines the modular CSS architecture used throughout the Cybrdelic Portfolio project.

## CSS Directory Structure

The CSS is organized into a modular structure with consistent naming and organization:

```
/static/css/
├── variables/            # CSS custom properties
│   └── variables.css     # All variables centralized in one file
├── base/                 # Core styles
│   ├── reset.css         # CSS reset and base element styles
│   ├── typography.css    # Typography and text styles
│   ├── utilities.css     # Utility classes for spacing, layout, etc.
│   └── animations.css    # Animation keyframes and utility classes
├── components/           # Reusable UI components 
│   ├── navigation.css    # Navigation bar and menu styles
│   ├── buttons.css       # Button styles and variations
│   ├── cards.css         # Card components
│   └── theme-toggle.css  # Theme toggle button
├── layouts/              # Layout patterns
│   └── scroll-snap.css   # Scroll snapping behavior
├── main.css              # Main CSS file that imports all modules
├── hero.css              # Legacy section CSS (to be refactored)
├── projects.css          # Legacy section CSS (to be refactored)
└── [other legacy files]  # Other legacy section CSS files
```

## How It Works

### 1. CSS Variables System

All design tokens are centralized in `variables/variables.css` for easy maintenance:

- **Colors**: Base colors, text colors, UI colors
- **Typography**: Font families, sizes, weights, line heights
- **Spacing**: Consistent spacing scale
- **Layout**: Grid, containers, breakpoints
- **Effects**: Transitions, shadows, filters
- **Animation**: Duration, timing functions

### 2. Dark Theme Support

The architecture supports theming through CSS variables. Simply toggle the `data-theme` attribute:

```html
<html data-theme="dark">
```

This triggers all themed variables to update automatically.

### 3. Component-Based Approach

Components are self-contained and use variables for consistency. For example:

```css
.card {
  background: var(--bg-glass);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius);
  padding: var(--space-lg);
  box-shadow: var(--shadow-glass);
}
```

### 4. Utility Classes

The architecture includes utility classes for common CSS patterns:

- Spacing: `.mt-md`, `.p-lg` 
- Layout: `.flex`, `.grid-2`
- Text: `.text-center`, `.text-bold`
- etc.

## Using the Architecture

1. Link to the main CSS file in your HTML:
```html
<link rel="stylesheet" href="/static/css/main.css" />
```

2. To add new components:
   - Create a new file in the appropriate directory
   - Add the import to main.css

3. For theme switching, use the JavaScript provided in base.html.

## Best Practices

1. Always use CSS variables for values that might be reused
2. Keep components isolated and focused on a single responsibility 
3. Use utility classes for one-off styling needs
4. Add new components to the appropriate directory
5. Update this documentation when adding new patterns

## Legacy CSS

Some CSS files are still in the root directory for backward compatibility. These will gradually be refactored into the modular architecture.