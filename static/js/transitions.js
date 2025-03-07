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
    
    // Make sure the overlay is not active on page load
    overlay.classList.remove('active');
    
    // Store current transition state in session storage
    if (sessionStorage.getItem('pageIsTransitioning') === 'true') {
        // Clean up after navigation
        sessionStorage.removeItem('pageIsTransitioning');
        // Remove overlay with a tiny delay to ensure clean transition
        setTimeout(() => {
            overlay.classList.remove('active');
        }, 10);
    }
    
    function handleNavigation(event) {
        const link = event.target.closest('[data-transition]');
        if (!link) return;
        
        event.preventDefault();
        const url = link.href;
        
        // Start transition
        overlay.classList.add('active');
        
        // Mark that we're in a transition
        sessionStorage.setItem('pageIsTransitioning', 'true');
        
        // Navigate after a very brief delay to allow overlay to appear
        setTimeout(() => {
            window.location.href = url;
        }, 10);
    }
    
    // Handle navigation events
    document.addEventListener('click', handleNavigation);
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
