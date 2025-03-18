// Mobile-specific JavaScript functionality

document.addEventListener("DOMContentLoaded", function() {
    // Only run on mobile devices
    if (window.innerWidth > 768) return;
    
    // =========================
    // Mobile navigation handling
    // =========================
    const navLinks = document.querySelectorAll('.nav a.cmd');
    const sections = document.querySelectorAll('section[id]');
    
    // Update active tab based on visible section
    function updateActiveTab() {
        let current = '';
        
        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            
            if (window.scrollY >= (sectionTop - 100) && 
                window.scrollY < (sectionTop + sectionHeight - 100)) {
                current = `#${section.getAttribute('id')}`;
            }
        });
        
        navLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === current) {
                link.classList.add('active');
            }
        });
    }
    
    // Listen for scroll events
    window.addEventListener('scroll', updateActiveTab);
    
    // =========================
    // Floating Action Button
    // =========================
    const fab = document.getElementById('mobile-fab');
    
    // Show/hide FAB based on scroll position
    window.addEventListener('scroll', function() {
        if (window.scrollY > 300) {
            fab.style.display = 'flex';
            fab.style.opacity = '1';
        } else {
            fab.style.opacity = '0';
            setTimeout(() => {
                if (window.scrollY <= 300) {
                    fab.style.display = 'none';
                }
            }, 300);
        }
    });
    
    // Scroll to top when FAB is clicked
    fab.addEventListener('click', function() {
        window.scrollTo({
            top: 0,
            behavior: 'smooth'
        });
    });
    
    // =========================
    // Pull-to-refresh indicator
    // =========================
    const pullIndicator = document.getElementById('pull-indicator');
    let touchStartY = 0;
    let isPulling = false;
    
    document.addEventListener('touchstart', function(e) {
        // Only trigger at the top of the page
        if (window.scrollY === 0) {
            touchStartY = e.touches[0].clientY;
            isPulling = true;
        }
    });
    
    document.addEventListener('touchmove', function(e) {
        if (!isPulling) return;
        
        const touchY = e.touches[0].clientY;
        const pullDistance = touchY - touchStartY;
        
        if (pullDistance > 0 && pullDistance < 100) {
            const progress = pullDistance / 100;
            pullIndicator.style.transform = `scaleX(${progress})`;
            pullIndicator.classList.add('active');
        }
    });
    
    document.addEventListener('touchend', function() {
        if (isPulling) {
            pullIndicator.style.transform = 'scaleX(0)';
            pullIndicator.classList.remove('active');
            isPulling = false;
        }
    });
    
    // =========================
    // Mobile-specific carousel behaviors
    // =========================
    
    // Horizontal scrolling carousels
    const carousels = document.querySelectorAll('.projects-list, #career .timeline, .selected-project-tech-tags, .project-tech, .tech-icons-container');
    
    carousels.forEach(carousel => {
        // Add visual indicator for scrollable content
        const indicator = document.createElement('div');
        indicator.className = 'scroll-indicator';
        indicator.innerHTML = '<i class="material-icons">swipe</i>';
        
        carousel.parentNode.style.position = 'relative';
        carousel.parentNode.appendChild(indicator);
        
        // Hide indicator after first scroll
        carousel.addEventListener('scroll', function() {
            indicator.style.opacity = '0';
            setTimeout(() => {
                indicator.style.display = 'none';
            }, 300);
        }, { once: true });
        
        // Snap scrolling for card-based carousels
        if (carousel.classList.contains('projects-list') || carousel.classList.contains('timeline')) {
            const cards = carousel.children;
            let startX, scrollLeft;
            
            carousel.addEventListener('touchstart', function(e) {
                startX = e.touches[0].pageX - carousel.offsetLeft;
                scrollLeft = carousel.scrollLeft;
            });
            
            carousel.addEventListener('touchend', function() {
                const cardWidth = cards[0].offsetWidth;
                const scrollPosition = carousel.scrollLeft;
                const cardIndex = Math.round(scrollPosition / cardWidth);
                
                carousel.scrollTo({
                    left: cardIndex * cardWidth,
                    behavior: 'smooth'
                });
            });
        }
    });
    
    // =========================
    // Enhanced touch feedback
    // =========================
    const touchElements = document.querySelectorAll('.cmd, .project-list-item, .project-card, .tech-tag, .details-link, #career .timeline-item');
    
    touchElements.forEach(element => {
        element.addEventListener('touchstart', function() {
            this.classList.add('touch-active');
        });
        
        element.addEventListener('touchend', function() {
            this.classList.remove('touch-active');
        });
        
        element.addEventListener('touchcancel', function() {
            this.classList.remove('touch-active');
        });
    });
    
    // Initialize on load
    updateActiveTab();
    
    // Hide FAB initially
    fab.style.display = 'none';
    fab.style.opacity = '0';
});