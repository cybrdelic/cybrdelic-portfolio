/* =========================================
* TRANSITIONS AND ANIMATIONS
========================================= */

/**
 * Manages smooth transitions between sections and animation effects
 * throughout the site for a cohesive experience.
 */

document.addEventListener('DOMContentLoaded', () => {
    // Initialize section transitions
    initSectionTransitions();
    
    // Set up page transitions
    initPageTransitions();
    
    // Initialize page-specific scripts
    initializePageScripts();
});

/**
 * Sets up intersection observers to trigger animations when
 * elements enter the viewport
 */
function initSectionTransitions() {
    // Detect elements with fade-in class
    const fadeElements = document.querySelectorAll('.fade-in');
    if (fadeElements.length > 0) {
        const fadeObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('visible');
                    // Only trigger once
                    fadeObserver.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1 });
        
        fadeElements.forEach(el => fadeObserver.observe(el));
    }
    
    // Detect section transitions
    const sections = document.querySelectorAll('section[id]');
    if (sections.length > 0) {
        const sectionObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('in-view');
                } else {
                    entry.target.classList.remove('in-view');
                }
            });
        }, { threshold: 0.1 });
        
        sections.forEach(section => sectionObserver.observe(section));
    }
}

/**
 * Sets up page transition effects for navigation between pages
 */
function initPageTransitions() {
    const overlay = document.getElementById('transition-overlay');
    const content = document.getElementById('content');
    
    // If no overlay or content elements exist, don't set up transitions
    if (!overlay || !content) return;
    
    async function handleNavigation(event) {
        const link = event.target.closest('[data-transition]');
        if (!link) return;
        
        event.preventDefault();
        const url = link.href;
        
        // Start transition
        overlay.classList.add('active');
        await new Promise(r => setTimeout(r, 300));
        
        try {
            // Fetch new content
            const response = await fetch(url);
            const html = await response.text();
            
            // Parse new content
            const parser = new DOMParser();
            const doc = parser.parseFromString(html, 'text/html');
            const newContent = doc.querySelector('#content').innerHTML;
            
            // Update URL and content
            window.history.pushState({}, '', url);
            content.innerHTML = newContent;
            
            // Update page class
            const newPageClass = doc.querySelector('body').className;
            document.body.className = newPageClass;
            
            // Initialize any scripts for new content
            initializePageScripts();
        } catch (error) {
            console.error('Navigation error:', error);
        }
        
        // End transition
        overlay.classList.remove('active');
    }
    
    // Handle navigation events
    document.addEventListener('click', handleNavigation);
    
    // Handle browser back/forward buttons
    window.addEventListener('popstate', async () => {
        if (!overlay || !content) return;
        
        overlay.classList.add('active');
        await new Promise(r => setTimeout(r, 300));
        
        try {
            const response = await fetch(window.location.href);
            const html = await response.text();
            const parser = new DOMParser();
            const doc = parser.parseFromString(html, 'text/html');
            
            if (doc.querySelector('#content')) {
                content.innerHTML = doc.querySelector('#content').innerHTML;
            }
            
            if (doc.querySelector('body')) {
                document.body.className = doc.querySelector('body').className;
            }
            
            initializePageScripts();
        } catch (error) {
            console.error('Navigation error on popstate:', error);
        }
        
        overlay.classList.remove('active');
    });
}

/**
 * Apply glitch effect to text elements
 * @param {HTMLElement} element - DOM element to apply effect to
 * @param {string} newText - Text to display after glitch
 * @param {number} duration - Duration of effect in ms
 */
function glitchText(element, newText, duration = 800) {
    if (!element) return;
    
    const chars = "!<>-_\\/[]{}—=+*^?#________";
    const originalText = element.textContent;
    let interval = null;
    let frame = 0;
    const frameRate = 30;
    const framesToComplete = duration / (1000 / frameRate);
    
    clearInterval(interval);
    interval = setInterval(() => {
        const progress = frame / framesToComplete;
        
        if (progress >= 1) {
            clearInterval(interval);
            element.textContent = newText;
            return;
        }
        
        const result = newText.split("").map((char, index) => {
            if (index < Math.floor(progress * newText.length)) {
                return newText[index];
            }
            
            return chars[Math.floor(Math.random() * chars.length)];
        }).join("");
        
        element.textContent = result;
        frame++;
    }, 1000 / frameRate);
}

/**
 * Initialize page-specific interactive elements
 */
function initializePageScripts() {
    // Projects hover effects
    const projects = document.querySelectorAll('.project');
    projects.forEach(project => {
        project.addEventListener('mouseenter', () => {
            if (Math.random() < 0.3) { // 30% chance of glitch effect
                project.classList.add('glitch');
                setTimeout(() => project.classList.remove('glitch'), 200);
            }
        });
    });
    
    // Add any other page-specific initialization here
    // This will be called on both initial load and after navigation
}

// Expose utility functions to global scope
window.glitchText = glitchText;
