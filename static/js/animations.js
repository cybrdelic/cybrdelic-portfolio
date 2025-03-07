/* =========================================
* GLOBAL ANIMATIONS AND MICROINTERACTIONS
========================================= */

document.addEventListener('DOMContentLoaded', () => {
    // Initialize all animations and interactions
    initScrollAnimations();
    initMicrointeractions();
    initCursorEffects();
    initThemeTransition();
});

/**
 * Sets up intersection observers for elements that should animate
 * when they enter the viewport
 */
function initScrollAnimations() {
    // Configuration for scroll animations
    const observerOptions = {
        root: null,
        rootMargin: '0px 0px -100px 0px',
        threshold: 0.15
    };

    // Main animation observer for simple animations
    const animateOnScroll = (entries, observer) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
                observer.unobserve(entry.target); // Only animate once
            }
        });
    };

    // Create and apply main observer
    const mainObserver = new IntersectionObserver(animateOnScroll, observerOptions);
    
    // Observe all simple animation elements
    const animatedElements = document.querySelectorAll('.fade-in, .fade-in-up, .fade-in-left, .fade-in-right, .animate-in-view');
    animatedElements.forEach(el => mainObserver.observe(el));
    
    // Handle staggered children animations
    const staggerParents = document.querySelectorAll('.stagger-children');
    
    if (staggerParents.length > 0) {
        const staggerObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    // Add visible class to parent to trigger child animations
                    entry.target.classList.add('in-view');
                    staggerObserver.unobserve(entry.target);
                }
            });
        }, observerOptions);
        
        staggerParents.forEach(el => staggerObserver.observe(el));
    }
    
    // Section animations
    const sections = document.querySelectorAll('section[id]');
    if (sections.length > 0) {
        const sectionObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('in-view');
                } else {
                    // Optional: remove the class when section is not in view
                    // entry.target.classList.remove('in-view');
                }
            });
        }, { threshold: 0.2 });
        
        sections.forEach(section => sectionObserver.observe(section));
    }
}

/**
 * Sets up microinteractions for interactive elements throughout the site
 */
function initMicrointeractions() {
    // Text scramble effect on hover
    document.querySelectorAll('.scramble-on-hover').forEach(el => {
        // Save original text as data attribute if not already set
        if (!el.getAttribute('data-text')) {
            el.setAttribute('data-text', el.textContent);
        }
    });
    
    // Command prompt hover effect
    document.querySelectorAll('.cmd-prompt').forEach(el => {
        el.addEventListener('mouseenter', () => {
            // Add a subtle 'active' state to command items on hover
            el.classList.add('cmd-active');
        });
        
        el.addEventListener('mouseleave', () => {
            el.classList.remove('cmd-active');
        });
    });
    
    // Link underline hover effect
    document.querySelectorAll('.link-underline').forEach(el => {
        // Ensure we're only applying to elements without explicit handlers
        if (!el.classList.contains('has-hover-handler')) {
            el.classList.add('has-hover-handler');
            
            // Follow mouse position for underline effect on larger screens
            if (window.innerWidth > 768) {
                el.addEventListener('mousemove', (e) => {
                    // Calculate percentage of mouse position within element
                    const rect = el.getBoundingClientRect();
                    const x = e.clientX - rect.left;
                    const width = rect.width;
                    const percentage = Math.min(Math.max(x / width, 0), 1);
                    
                    // Apply custom property for CSS to use
                    el.style.setProperty('--underline-width', `${percentage * 100}%`);
                });
                
                el.addEventListener('mouseleave', () => {
                    // Reset on mouse leave
                    el.style.setProperty('--underline-width', '0%');
                });
            }
        }
    });
}

/**
 * Initialize subtle cursor-based effects for enhanced interactivity
 */
function initCursorEffects() {
    // Only apply on non-touch devices
    if ('ontouchstart' in window) return;
    
    let mouseX = 0;
    let mouseY = 0;
    let frameRequest;
    
    // Track mouse position
    document.addEventListener('mousemove', (e) => {
        mouseX = e.clientX;
        mouseY = e.clientY;
        
        // Use requestAnimationFrame for better performance
        if (!frameRequest) {
            frameRequest = requestAnimationFrame(updateElements);
        }
    });
    
    function updateElements() {
        frameRequest = null;
        
        // Find card elements that should react to cursor
        document.querySelectorAll('.card-hover').forEach(el => {
            const rect = el.getBoundingClientRect();
            
            // Check if mouse is near the element (within 150px)
            const centerX = rect.left + rect.width / 2;
            const centerY = rect.top + rect.height / 2;
            const distX = mouseX - centerX;
            const distY = mouseY - centerY;
            const distance = Math.sqrt(distX * distX + distY * distY);
            
            // Apply subtle transform based on cursor proximity
            if (distance < 200) {
                const intensity = 1 - Math.min(distance / 200, 1);
                const tiltX = distY * intensity * 0.02;
                const tiltY = -distX * intensity * 0.02;
                
                el.style.transform = `perspective(1000px) rotateX(${tiltX}deg) rotateY(${tiltY}deg) translateZ(5px)`;
                el.style.transition = 'transform 0.2s ease-out';
            } else {
                el.style.transform = '';
                el.style.transition = 'transform 0.5s ease';
            }
        });
    }
}

/**
 * Set up smooth transitions between light and dark themes
 */
function initThemeTransition() {
    // Corner effects animation on theme change
    const cornerEffects = document.querySelectorAll('.corner-effect');
    const themeToggle = document.getElementById('nav-theme-toggle');
    
    if (themeToggle) {
        themeToggle.addEventListener('click', () => {
            // Animate corner effects
            cornerEffects.forEach(corner => {
                // Create a subtle pulse animation
                corner.style.opacity = '0.5';
                setTimeout(() => {
                    corner.style.opacity = '0.3';
                }, 300);
            });
        });
    }
}

/**
 * Scramble text effect for an element
 */
function scrambleText(element, newText, duration = 800) {
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

// Expose animation utilities to global scope
window.scrambleText = scrambleText;
