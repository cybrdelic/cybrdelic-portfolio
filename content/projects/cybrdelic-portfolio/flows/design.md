# Design System

Cybrdelic Portfolio incorporates a comprehensive design system that ensures visual consistency while providing flexibility for creative expression. This document outlines the key design principles and components.

## Design Principles

The design system follows four core principles:

1. **Cyberpunk Aesthetic with Professional Polish**
   - Bold, high-contrast visuals balanced with professional readability
   - Futuristic elements that don't sacrifice usability

2. **Terminal-Inspired Interaction**
   - Command-line inspired navigation and feedback
   - Typewriter and glitch effects that feel tactile

3. **Performance-First Approach**
   - Minimal dependencies for fast loading
   - Progressive enhancement for broader device support

4. **Accessibility Without Compromise**
   - Maintains WCAG compliance while preserving visual style
   - Keyboard navigation and screen reader support

## Color System

The color system is built around a carefully selected palette:

### Primary Colors

```
--color-black: #000000
--color-white: #ffffff
--color-accent: #55aaff
```

### Theme Variables

Colors are abstracted into semantic variables:

```css
/* Light theme */
--color-text-primary: #000000;
--color-text-secondary: #222222;
--body-bg: white;

/* Dark theme */
[data-theme='dark'] {
  --color-text-primary: #ffffff;
  --color-text-secondary: #dddddd;
  --body-bg: #070707;
}
```

### Glass Effects

Special glass-like surfaces use backdrop filters:

```css
--bg-glass: rgba(255, 255, 255, 0.15);
--bg-glass-hover: rgba(255, 255, 255, 0.2);
```

## Typography System

### Font Families

The portfolio uses a carefully selected set of font families:

```css
--font-primary: 'IBM Plex Mono', monospace;
--font-display: 'Syne', sans-serif;
--font-mono: 'JetBrains Mono', 'IBM Plex Mono', monospace;
```

### Font Sizing

Typography follows a modular scale:

```css
--font-size-2xs: 0.7rem;
--font-size-xs: 0.875rem;
--font-size-sm: 1rem;
--font-size-base: 1.125rem;
--font-size-md: 1.25rem;
--font-size-lg: 1.5rem;
--font-size-xl: 2rem;
--font-size-2xl: 3rem;
```

### Fluid Typography

For responsive headings:

```css
--font-size-fluid-sm: 4vw;
--font-size-fluid-base: 5vw;
--font-size-fluid-lg: 7.5vw;
--font-size-fluid-xl: 10vw;
```

## Space Scale

Spacing follows a consistent scale:

```css
--space-2xs: 4px;
--space-xs: 8px;
--space-sm: 12px;
--space-md: 16px;
--space-lg: 24px;
--space-xl: 32px;
--space-2xl: 40px;
```

## Component Library

### Command Prompts

```html
<a href="#projects" class="cmd cmd-prompt">ls projects/</a>
```

Command prompts use a terminal-like styling:

```css
.cmd {
  font-family: var(--font-mono);
  padding: var(--space-xs) var(--space-md);
  background: var(--bg-glass);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius);
}
```

### Cards

Cards provide contained content areas with consistent styling:

```css
.card {
  background: var(--bg-glass);
  backdrop-filter: blur(10px);
  border: 1px solid var(--color-border);
  border-radius: var(--border-radius);
  transition: all 0.3s ease;
}

.card:hover {
  transform: translateY(-5px);
  box-shadow: var(--shadow-lg);
}
```

### Buttons

Buttons follow a consistent pattern:

```css
.button {
  display: inline-flex;
  align-items: center;
  padding: var(--space-sm) var(--space-lg);
  font-family: var(--font-primary);
  border-radius: var(--border-radius);
  transition: all 0.3s ease;
}

.primary-button {
  background: var(--color-text-primary);
  color: var(--body-bg);
}

.secondary-button {
  background: transparent;
  border: 1px solid var(--color-border);
}
```

## Animation System

Animations use CSS variables for consistent timing:

```css
--transition-fast: 0.15s ease;
--transition-base: 0.2s ease;
--transition-medium: 0.3s ease;
--transition-slow: 0.6s cubic-bezier(0.19, 1, 0.22, 1);
```

### Animation Types

1. **Micro-interactions**
   - Hover states
   - Focus states
   - Active states

2. **Page Transitions**
   - Content fade-in
   - Section reveals

3. **Text Effects**
   - Glitch animations
   - Typing effects
   - Scrambling text

### Example Implementation

```css
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.fade-in {
  animation: fadeIn 0.5s var(--transition-slow) forwards;
}
```

## Responsive System

The layout adapts to different screen sizes:

```css
/* Base styles for mobile */

/* Tablet */
@media (min-width: 768px) {
  /* Tablet specific styles */
}

/* Desktop */
@media (min-width: 1024px) {
  /* Desktop specific styles */
}

/* Large desktop */
@media (min-width: 1280px) {
  /* Large desktop specific styles */
}
```

## Accessibility Considerations

- Color contrast meets WCAG AA standards
- Focus states are clearly visible
- All interactive elements are keyboard accessible
- Images have appropriate alt text
- Animations can be disabled via `prefers-reduced-motion`