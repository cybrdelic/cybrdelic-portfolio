// Project interaction and animation optimizations
document.addEventListener('DOMContentLoaded', function() {
    // Debounce function to limit performance impact of scroll/resize events
    function debounce(func, wait) {
        let timeout;
        return function() {
            const context = this, args = arguments;
            clearTimeout(timeout);
            timeout = setTimeout(function() {
                func.apply(context, args);
            }, wait);
        };
    }

    // Cache DOM elements to improve performance
    const projectCards = document.querySelectorAll('.project-card');
    const projectItems = document.querySelectorAll('.project-list-item');
    
    // Use IntersectionObserver for better performance than scroll handlers
    const elementObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('reveal');
                elementObserver.unobserve(entry.target);
            }
        });
    }, {
        root: null,
        rootMargin: '0px',
        threshold: 0.1
    });

    // Observe all project cards and list items
    projectCards.forEach(card => elementObserver.observe(card));
    projectItems.forEach(item => elementObserver.observe(item));

    // Handle project card hover interactions with throttling
    projectCards.forEach(card => {
        card.addEventListener('mouseenter', debounce(function() {
            // Add hover class to card
            card.classList.add('hover');
            // Prefetch any images or resources needed on hover
            const images = card.querySelectorAll('img[data-src]');
            images.forEach(img => {
                if (img.dataset.src) {
                    img.src = img.dataset.src;
                    img.removeAttribute('data-src');
                }
            });
        }, 50));

        card.addEventListener('mouseleave', debounce(function() {
            card.classList.remove('hover');
        }, 50));
    });

    // Handle theme changes for proper styling
    function updateThemeSpecificStyles() {
        const theme = document.documentElement.getAttribute('data-theme') || 'light';
        document.body.setAttribute('data-theme', theme);
    }

    // Watch for theme changes
    const themeObserver = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.attributeName === 'data-theme') {
                updateThemeSpecificStyles();
            }
        });
    });

    themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['data-theme']
    });

    // Initialize theme styles
    updateThemeSpecificStyles();
});