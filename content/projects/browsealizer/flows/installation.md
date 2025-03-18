# Installation Guide

Browsealizer can be installed and used in multiple ways. Choose the method that works best for your workflow.

## Web Application

The simplest way to use Browsealizer is through our web application:

1. Visit [https://browsealizer.dev](https://browsealizer.dev) in any modern browser
2. Create an account or sign in with GitHub
3. Start browsing repositories immediately

The web application offers the full Browsealizer experience with no installation required.

## Progressive Web App (PWA)

For a more integrated experience, install Browsealizer as a Progressive Web App:

### Desktop Installation

1. Visit [https://browsealizer.dev](https://browsealizer.dev) in Chrome, Edge, or another Chromium-based browser
2. Click the install icon (➕) in the address bar
3. Select "Install" in the prompt
4. Browsealizer will appear in your application menu/dock

### Mobile Installation

#### Android
1. Visit [https://browsealizer.dev](https://browsealizer.dev) in Chrome
2. Tap the menu button (⋮)
3. Select "Add to Home screen"
4. Confirm by tapping "Add"

#### iOS
1. Visit [https://browsealizer.dev](https://browsealizer.dev) in Safari
2. Tap the share button (□↑)
3. Scroll down and select "Add to Home Screen"
4. Tap "Add" in the confirmation dialog

## Chrome Extension

Our Chrome extension integrates Browsealizer directly into GitHub:

```bash
# Installation steps
1. Visit the Chrome Web Store
2. Search for "Browsealizer"
3. Click "Add to Chrome"
4. Confirm the installation
```

The extension adds Browsealizer features directly to GitHub pages:

- Related repository suggestions on repository pages
- Enhanced search capabilities on GitHub search pages
- Quick filters in your GitHub feed
- Repository insights integrated into GitHub UI

## Firefox Add-on

For Firefox users, we offer a compatible add-on:

1. Visit the Firefox Add-ons Marketplace
2. Search for "Browsealizer"
3. Click "Add to Firefox"
4. Follow the prompts to complete installation

## Self-Hosted Version

For organizations or privacy-focused users, we offer a self-hosted version:

### Docker Installation

```bash
# Pull the Docker image
docker pull cybrdelic/browsealizer:latest

# Run the container
docker run -p 3000:3000 cybrdelic/browsealizer:latest
```

Access the application at `http://localhost:3000`

### Manual Installation

```bash
# Clone the repository
git clone https://github.com/cybrdelic/browsealizer.git

# Navigate to the project directory
cd browsealizer

# Install dependencies
npm install

# Build the application
npm run build

# Start the server
npm start
```

## System Requirements

Browsealizer is designed to work on most modern devices:

### Web Application
- Any modern browser (Chrome, Firefox, Safari, Edge)
- JavaScript enabled
- 2GB RAM minimum (4GB recommended)
- 100MB free disk space for caching

### Mobile Application
- iOS 13+ or Android 8+
- 200MB free storage
- Data connection for initial load and repository fetching

### Self-Hosted Version
- Node.js 14+
- 2GB RAM minimum
- 500MB free disk space
- Network access to GitHub API endpoints