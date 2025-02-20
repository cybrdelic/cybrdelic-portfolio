#!/usr/bin/env bash

output_file="source-code-collection.txt"

echo "source code content - created on $(date)" >"$output_file"
echo "================================================" >>"$output_file"

append_files() {
    local file_type="$1"
    local pattern="$2"
    echo -e "\n[$file_type files]\n" >>"$output_file"
    find . \
        \( -path './target' -o -path './node_modules' -o -path './dist' -o -path './build' -o -path './.git' -o -path './__pycache__' -o -path './.cargo' \) \
        -prune -o -type f -name "$pattern" -print | while read -r file; do
        rel_path="${file#./}"
        echo -e "\n--- $rel_path ---\n" >>"$output_file"
        cat "$file" >>"$output_file"
        echo >>"$output_file"
    done
}

echo "collecting rust files..."
append_files "rust" "*.rs"

echo "collecting html files..."
append_files "html" "*.html"

echo "collecting css files..."
append_files "css" "*.css"

echo "collecting javascript files..."
append_files "javascript" "*.js"

rust_count=$(find . \( -path './target' -o -path './node_modules' -o -path './dist' -o -path './build' -o -path './.git' -o -path './__pycache__' -o -path './.cargo' \) -prune -o -type f -name '*.rs' -print | wc -l)
html_count=$(find . \( -path './target' -o -path './node_modules' -o -path './dist' -o -path './build' -o -path './.git' -o -path './__pycache__' -o -path './.cargo' \) -prune -o -type f -name '*.html' -print | wc -l)
css_count=$(find . \( -path './target' -o -path './node_modules' -o -path './dist' -o -path './build' -o -path './.git' -o -path './__pycache__' -o -path './.cargo' \) -prune -o -type f -name '*.css' -print | wc -l)
js_count=$(find . \( -path './target' -o -path './node_modules' -o -path './dist' -o -path './build' -o -path './.git' -o -path './__pycache__' -o -path './.cargo' \) -prune -o -type f -name '*.js' -print | wc -l)

echo -e "\nsummary" >>"$output_file"
echo "-------" >>"$output_file"
echo "rust files (.rs): $rust_count" >>"$output_file"
echo "html files: $html_count" >>"$output_file"
echo "css files: $css_count" >>"$output_file"
echo "javascript files: $js_count" >>"$output_file"
echo "total files: $((rust_count + html_count + css_count + js_count))" >>"$output_file"

clipboard_command=""
if command -v xclip >/dev/null; then
    clipboard_command="xclip -selection clipboard"
elif command -v xsel >/dev/null; then
    clipboard_command="xsel --clipboard --input"
elif command -v pbcopy >/dev/null; then
    clipboard_command="pbcopy"
elif command -v clip.exe >/dev/null; then
    clipboard_command="clip.exe"
fi

if [ -n "$clipboard_command" ]; then
    cat "$output_file" | $clipboard_command
    echo "source code content has been copied to clipboard"
    echo "files processed:"
    echo "- rust files (.rs): $rust_count"
    echo "- html files: $html_count"
    echo "- css files: $css_count"
    echo "- javascript files: $js_count"
    echo "total files: $((rust_count + html_count + css_count + js_count))"
else
    echo "no clipboard command found. content saved in: $output_file"
fi
