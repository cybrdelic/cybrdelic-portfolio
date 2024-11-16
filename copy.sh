#!/bin/bash

# Directories to exclude
exclude_dirs=(
    "target"       # Rust build artifacts
    "node_modules" # Node.js packages
    "dist"         # Common build output
    "build"        # Common build output
    ".git"         # Git directory
    "__pycache__"  # Python cache
    ".cargo"       # Cargo cache
)

# Build the exclude pattern for find command
exclude_pattern=""
for dir in "${exclude_dirs[@]}"; do
    exclude_pattern="$exclude_pattern -not -path './$dir/*'"
done

# Function to copy to clipboard based on OS
copy_to_clipboard() {
    if command -v xclip >/dev/null 2>&1; then
        xclip -selection clipboard
    elif command -v xsel >/dev/null 2>&1; then
        xsel --clipboard --input
    elif command -v pbcopy >/dev/null 2>&1; then
        pbcopy
    elif command -v clip.exe >/dev/null 2>&1; then
        clip.exe
    else
        echo "No clipboard command found. Please install xclip, xsel, or pbcopy."
        return 1
    fi
}

# Create a temporary file to store the content
temp_file=$(mktemp)

# Write header
echo "Source Code Content - Created on $(date)" >"$temp_file"
echo "================================================" >>"$temp_file"

# Function to append files of a specific type
append_files() {
    local file_type=$1
    local pattern=$2

    echo -e "\n[$file_type Files]\n" >>"$temp_file"

    while IFS= read -r file; do
        # Get relative path
        rel_path=$(realpath --relative-to="." "$file")

        # Add file header
        echo -e "\n--- $rel_path ---\n" >>"$temp_file"

        # Add file content
        cat "$file" >>"$temp_file"
        echo -e "\n" >>"$temp_file"

    done < <(eval "find . -type f -name \"$pattern\" $exclude_pattern")
}

# Collect all source files
echo "Collecting Rust files..."
append_files "Rust" "*.rs"

echo "Collecting HTML files..."
append_files "HTML" "*.html"

echo "Collecting CSS files..."
append_files "CSS" "*.css"

echo "Collecting JavaScript files..."
append_files "JavaScript" "*.js"

# Count files processed
rust_count=$(eval "find . -type f -name '*.rs' $exclude_pattern" | wc -l)
html_count=$(eval "find . -type f -name '*.html' $exclude_pattern" | wc -l)
css_count=$(eval "find . -type f -name '*.css' $exclude_pattern" | wc -l)
js_count=$(eval "find . -type f -name '*.js' $exclude_pattern" | wc -l)

# Add summary
echo -e "\nSummary" >>"$temp_file"
echo "-------" >>"$temp_file"
echo "Rust files (.rs): $rust_count" >>"$temp_file"
echo "HTML files: $html_count" >>"$temp_file"
echo "CSS files: $css_count" >>"$temp_file"
echo "JavaScript files: $js_count" >>"$temp_file"
echo "Total files: $((rust_count + html_count + css_count + js_count))" >>"$temp_file"

# Copy to clipboard
if cat "$temp_file" | copy_to_clipboard; then
    echo "Source code content has been copied to clipboard"
    echo "Files processed:"
    echo "- Rust files (.rs): $rust_count"
    echo "- HTML files: $html_count"
    echo "- CSS files: $css_count"
    echo "- JavaScript files: $js_count"
    echo "Total files: $((rust_count + html_count + css_count + js_count))"
else
    echo "Failed to copy to clipboard. Content saved in: $temp_file"
fi

# Clean up
rm "$temp_file"
