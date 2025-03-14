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
    
    // Initialize navbar scroll behavior
    initNavbarScrollBehavior();
    
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
 * with instant loading and progressive enhancement
 */
function initPageTransitions() {
    const overlay = document.getElementById('transition-overlay');
    const content = document.getElementById('content');
    
    // If no overlay or content elements exist, don't set up transitions
    if (!overlay || !content) return;
    
    // Just remove active class for faster render, but keep element visible
    overlay.classList.remove('active');
    
    // Check if we're coming from a transition
    if (sessionStorage.getItem('pageIsTransitioning') === 'true') {
        sessionStorage.removeItem('pageIsTransitioning');
    }
    
    // Progressive enhancement:
    // 1. First render the page immediately
    // 2. Then load dynamic content behind the scenes
    function progressivelyEnhancePage() {
        // This is for project pages only
        if (window.location.pathname.includes('/projects/')) {
            // Load any markdown content asynchronously
            const markdownContainers = document.querySelectorAll('.markdown-content');
            if (markdownContainers.length > 0) {
                // Add a subtle loading indicator
                markdownContainers.forEach(container => {
                    if (container.innerHTML.trim() === '') {
                        container.innerHTML = '<div class="loading-pulse">Loading documentation...</div>';
                    }
                });
                
                // Initialize any necessary components
                if (typeof mermaid !== 'undefined') {
                    setTimeout(() => {
                        mermaid.init(undefined, document.querySelectorAll('.language-mermaid'));
                    }, 100);
                }
            }
        }
    }
    
    // Run progressive enhancement on page load
    progressivelyEnhancePage();
    
    // Handle navigation with instant transitions
    function handleNavigation(event) {
        const link = event.target.closest('[data-transition]');
        if (!link) return;
        
        event.preventDefault();
        const url = link.href;
        
        // Just navigate directly for instant page loads
        window.location.href = url;
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

/**
 * Handle navbar scroll behavior - hide on scroll down, show on scroll up
 */
function initNavbarScrollBehavior() {
    const navbar = document.querySelector('.nav');
    if (!navbar) return;
    
    // Add visible class initially
    navbar.classList.add('nav-visible');
    
    let lastScrollTop = 0;
    let scrollThreshold = 50; // Minimum scroll amount to trigger change
    let scrollTimeout;
    
    // Throttled scroll handler
    window.addEventListener('scroll', () => {
        // Clear existing timeout
        if (scrollTimeout) clearTimeout(scrollTimeout);
        
        // Set a new timeout to run code after scrolling stops for 50ms
        scrollTimeout = setTimeout(() => {
            const currentScrollTop = window.pageYOffset || document.documentElement.scrollTop;
            
            // Skip if not scrolled enough to meet threshold
            if (Math.abs(lastScrollTop - currentScrollTop) < scrollThreshold) return;
            
            // If scrolled down and past the threshold, hide the navbar
            if (currentScrollTop > lastScrollTop && currentScrollTop > 100) {
                navbar.classList.add('nav-hidden');
                navbar.classList.remove('nav-visible');
            } 
            // If scrolled up, show the navbar
            else if (currentScrollTop < lastScrollTop) {
                navbar.classList.remove('nav-hidden');
                navbar.classList.add('nav-visible');
            }
            
            lastScrollTop = currentScrollTop;
        }, 50);
    });
}

// Expose utility functions to global scope
window.glitchText = glitchText;
