# Installation Guide

JJugg is a web application that can be installed and run entirely in your browser. This guide will walk you through the different installation options.

## Option 1: Web App (Recommended)

The simplest way to use JJugg is through our web application:

1. Visit [https://jjugg.app](https://jjugg.app) in your browser
2. Click "Get Started" to create your account
3. Follow the onboarding process to set up your profile

The web app automatically saves your data to your browser's localStorage, so you can close the browser and return later without losing any information.

## Option 2: Progressive Web App (PWA)

For a more native app-like experience, you can install JJugg as a Progressive Web App:

1. Visit [https://jjugg.app](https://jjugg.app) in Chrome, Edge, or another supported browser
2. Look for the install icon (➕) in the address bar
3. Click "Install" when prompted
4. JJugg will now appear as an application in your start menu or dock

The PWA version works offline and provides notifications for application status updates.

## Option 3: Self-Hosted Version

For users who prefer complete control over their data, JJugg can be self-hosted:

```bash
# Clone the repository
git clone https://github.com/cybrdelic/jjugg.git

# Navigate to the project folder
cd jjugg

# Install dependencies
npm install

# Start the local development server
npm start
```

The self-hosted version runs on your local network at `http://localhost:3000` by default.

## Browser Extensions

To enhance JJugg's functionality, we recommend installing our browser extensions:

- [JJugg for Chrome](https://chrome.google.com/webstore/detail/jjugg/abcdefghijklmnop)
- [JJugg for Firefox](https://addons.mozilla.org/en-US/firefox/addon/jjugg/)

These extensions enable automatic job application detection and tracking across different job boards.

## System Requirements

JJugg works on any modern browser, including:

- Chrome (version 80+)
- Firefox (version 75+)
- Safari (version 14+)
- Edge (Chromium-based, version 80+)

For the best experience, we recommend using a desktop or laptop computer with at least:

- 4GB RAM
- Any modern processor
- 100MB free disk space (for the PWA version)