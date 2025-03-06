# Cybrdelic Portfolio - CSS Documentation

This document outlines the CSS structure used throughout the Cybrdelic Portfolio project.

## Current CSS Structure

The project currently uses a flat CSS structure with separate files for different sections:

```
/static/css/
├── base.css           # Core styles, variables, and reset
├── hero.css           # Hero section styling
├── projects.css       # Projects section styling
├── career.css         # Career timeline section styling
├── project-detail.css # Project detail page styling
├── project-hero.css   # Project hero section styling
├── documentation.css  # Documentation styling
├── scroll-snap.css    # Scroll snap behavior
└── README.md          # This documentation file
```

## Key CSS Variables

The project uses CSS custom properties extensively for consistency:

### Colors and Text
- Base colors: `--color-black`, `--color-white`, etc.
- Text colors: `--color-text-primary`, `--color-text-secondary`, etc.
- UI colors: `--color-border`, `--color-accent`, etc.

### Spacing
A consistent spacing scale using incremental values:
- `--space-2xs`: 4px
- `--space-xs`: 8px
- `--space-sm`: 12px
- `--space-md`: 16px
- `--space-lg`: 24px
- `--space-xl`: 32px
- `--space-2xl`: 40px

### Typography
- Font families: `--font-primary`, `--font-display`
- Font sizes: From `--font-size-xs` to `--font-size-xl` 
- Fluid typography: `--font-size-fluid-sm` to `--font-size-fluid-xl`

## Common Components

### Navigation
- `.nav`: Main navigation bar
- `.cmd`: Command-style navigation links
- `.logo`: Site logo styling

### Cards & Containers
- `.card`: Basic card component
- `.project-card`: Project display card
- `.console-description`: Description panel in project console

### Buttons
- `.btn`: Basic button styling
- `.availability`: Availability status indicator
- `.theme-toggle`: Theme switching button

## Future Improvements

### Potential Reorganization
A more modular approach could be beneficial for larger codebases:

```
/static/css/
├── core/           # Variables, reset, base styles
├── components/     # Reusable UI components
├── layout/         # Page layouts and sections
├── animations/     # Animation keyframes
└── main.css        # Import file
```

### Best Practices

1. Continue using CSS custom properties for consistency
2. Keep related styles together in semantic groupings
3. Maintain clear naming conventions
4. Consider adopting a utility-first approach for frequently used patterns
5. Document new patterns in this README