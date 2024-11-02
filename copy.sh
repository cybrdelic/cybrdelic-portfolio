#!/bin/bash

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for xclip (Linux) or pbcopy (macOS)
if ! command_exists xclip && ! command_exists pbcopy; then
    echo "Error: Neither xclip (Linux) nor pbcopy (macOS) found. Please install one of them."
    exit 1
fi

# Create temporary file
temp_file=$(mktemp)

# Function to copy to clipboard based on OS
copy_to_clipboard() {
    if command_exists xclip; then
        xclip -selection clipboard
    elif command_exists pbcopy; then
        pbcopy
    fi
}

{
    echo "Project: cybrdelic-portfolio"
    echo
    
    echo "### Cargo.toml ###"
    if [ -f "Cargo.toml" ]; then
        echo "'''toml"
        cat "Cargo.toml"
        echo "'''"
        echo
    fi
    
    echo "### src/main.rs ###"
    if [ -f "src/main.rs" ]; then
        echo "'''rust"
        cat "src/main.rs"
        echo "'''"
        echo
    fi
    
    echo "### static/index.html ###"
    if [ -f "static/index.html" ]; then
        echo "'''html"
        cat "static/index.html"
        echo "'''"
        echo
    fi
    
    echo "### static/project-template.html ###"
    if [ -f "static/project-template.html" ]; then
        echo "'''html"
        cat "static/project-template.html"
        echo "'''"
        echo
    fi
    
    echo "### static/styles/main.css ###"
    if [ -f "static/styles/main.css" ]; then
        echo "'''css"
        cat "static/styles/main.css"
        echo "'''"
        echo
    fi
    
    echo "### static/js/main.js ###"
    if [ -f "static/js/main.js" ]; then
        echo "'''javascript"
        cat "static/js/main.js"
        echo "'''"
        echo
    fi
    
    echo "Project Structure:"
    if command_exists tree; then
        tree -I 'target|.git'
    else
        find . -type f -not -path '*/\.*' -not -path '*/target/*'
    fi
    echo
    
    echo "Dependencies:"
    echo "- Rust backend using Axum"
    echo "- HTMX for dynamic content"
    echo "- Custom CSS for cyberpunk styling"
    echo "- JetBrains Mono & Space Grotesk fonts"
    echo "- Pure HTML/CSS/JS frontend (no framework)"
    echo
    
    echo "Key Features:"
    echo "- Server-side rendering"
    echo "- Project showcase with detailed pages"
    echo "- Terminal-inspired UI"
    echo "- Cyberpunk aesthetic"
    echo "- Responsive design"
    
} > "$temp_file"

# Copy to clipboard
cat "$temp_file" | copy_to_clipboard

# Get size of content
size=$(wc -c < "$temp_file")
lines=$(wc -l < "$temp_file")

# Show a preview of what was copied
echo "Preview of copied content:"
echo "------------------------"
head -n 5 "$temp_file"
echo "..."
tail -n 5 "$temp_file"
echo "------------------------"

# Clean up
rm "$temp_file"

echo "Project context copied to clipboard!"
echo "Size: $(($size/1024))KB"
echo "Lines: $lines"
