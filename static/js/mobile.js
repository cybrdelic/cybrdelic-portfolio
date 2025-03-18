// mobile.js - Enhanced mobile-specific JavaScript functionality

document.addEventListener("DOMContentLoaded", function() {
    // Only run on mobile devices or when testing on desktop with mobile width
    const isMobile = window.innerWidth <= 768;
    if (!isMobile) return;
    
    // =========================
    // Material Design Ripple Effect
    // =========================
    
    // Add tap-highlight divs to all interactive elements
    const interactiveElements = document.querySelectorAll(`
        .cmd, 
        .project-list-item,
        .project-card,
        .tech-tag,
        .details-link,
        #career .timeline-item,
        .project-tech-item,
        .expertise-tag,
        .topic-card,
        .mobile-fab,
        .project-meta-item,
        .project-cta-button
    `);
    
    interactiveElements.forEach(el => {
        // Create and append tap highlight element
        const tapHighlight = document.createElement('div');
        tapHighlight.className = 'tap-highlight';
        el.appendChild(tapHighlight);
        
        // Add ripple effect
        el.addEventListener('touchstart', createRipple);
        el.addEventListener('mousedown', createRipple); // For testing on desktop
    });
    
    function createRipple(event) {
        const element = event.currentTarget;
        
        // Remove any existing ripples
        const existingRipples = element.querySelectorAll('.ripple');
        existingRipples.forEach(ripple => {
            if (ripple.parentNode === element) {
                element.removeChild(ripple);
            }
        });
        
        // Create ripple element
        const ripple = document.createElement('span');
        ripple.className = 'ripple';
        element.appendChild(ripple);
        
        // Calculate position
        const rect = element.getBoundingClientRect();
        
        // Get position based on touch or mouse
        let x, y;
        if (event.touches && event.touches[0]) {
            x = event.touches[0].clientX - rect.left;
            y = event.touches[0].clientY - rect.top;
        } else {
            x = event.clientX - rect.left;
            y = event.clientY - rect.top;
        }
        
        // Calculate size (diagonal of the element to ensure it covers the entire element)
        const size = Math.max(rect.width, rect.height) * 2;
        
        // Position and size ripple
        ripple.style.width = ripple.style.height = `${size}px`;
        ripple.style.left = `${x - size/2}px`;
        ripple.style.top = `${y - size/2}px`;
        
        // Clean up ripple after animation completes
        setTimeout(() => {
            if (ripple.parentNode === element) {
                element.removeChild(ripple);
            }
        }, 600);
    }
    
    // =========================
    // Enhanced Snap Scrolling for Carousels
    // =========================
    
    // Find all horizontal carousels
    const carousels = document.querySelectorAll('.projects-list, #career .timeline, .related-projects');
    
    carousels.forEach(carousel => {
        let isScrolling = false;
        let startX;
        let startScrollLeft;
        let startTime;
        let targetScrollLeft;
        let animationFrameId;
        
        // Add visual indicator for scrollable content
        const indicator = document.createElement('div');
        indicator.className = 'scroll-indicator';
        indicator.innerHTML = '<i class="material-icons">swipe</i>';
        
        // Only add indicator if parent has position relative
        if (window.getComputedStyle(carousel.parentNode).position !== 'relative') {
            carousel.parentNode.style.position = 'relative';
        }
        carousel.parentNode.appendChild(indicator);
        
        // Hide indicator after first scroll
        carousel.addEventListener('scroll', function() {
            indicator.style.opacity = '0';
            setTimeout(() => {
                indicator.style.display = 'none';
            }, 300);
        }, { once: true });
        
        // Handle start of touch
        carousel.addEventListener('touchstart', function(e) {
            // Cancel any ongoing animations
            if (animationFrameId) {
                cancelAnimationFrame(animationFrameId);
            }
            
            isScrolling = true;
            startX = e.touches[0].pageX;
            startScrollLeft = carousel.scrollLeft;
            startTime = Date.now();
            
            // Add active class to indicate scrolling
            carousel.classList.add('carousel-scrolling');
            
            // Disable other event handlers while scrolling
            e.stopPropagation();
        });
        
        // Handle touchmove
        carousel.addEventListener('touchmove', function(e) {
            if (!isScrolling) return;
            
            // Calculate how far finger has moved
            const x = e.touches[0].pageX;
            const dx = startX - x;
            
            // Update scroll position
            carousel.scrollLeft = startScrollLeft + dx;
            
            // Disable other event handlers while scrolling
            e.stopPropagation();
        });
        
        // Handle end of touch - with momentum and snap
        carousel.addEventListener('touchend', function(e) {
            if (!isScrolling) return;
            
            isScrolling = false;
            const endTime = Date.now();
            const timeElapsed = endTime - startTime;
            const endScrollLeft = carousel.scrollLeft;
            const distance = endScrollLeft - startScrollLeft;
            
            // Calculate velocity (pixels per millisecond)
            const velocity = distance / timeElapsed;
            
            // Only add momentum if velocity is significant
            if (Math.abs(velocity) > 0.5) {
                // Calculate target position with momentum
                const momentum = velocity * 300; // Adjust this multiplier for stronger/weaker momentum
                targetScrollLeft = endScrollLeft + momentum;
            } else {
                targetScrollLeft = endScrollLeft;
            }
            
            // Find nearest snap point
            if (carousel.children.length > 0) {
                const cardWidth = carousel.children[0].offsetWidth + 
                                parseInt(window.getComputedStyle(carousel.children[0]).marginRight);
                const snapPoint = Math.round(targetScrollLeft / cardWidth) * cardWidth;
                targetScrollLeft = snapPoint;
            }
            
            // Ensure within bounds
            targetScrollLeft = Math.max(0, Math.min(targetScrollLeft, carousel.scrollWidth - carousel.clientWidth));
            
            // Animate to target with easing
            function animateScroll() {
                const currentPosition = carousel.scrollLeft;
                const distance = targetScrollLeft - currentPosition;
                
                // Exit if we're close enough
                if (Math.abs(distance) < 1) {
                    carousel.scrollLeft = targetScrollLeft;
                    carousel.classList.remove('carousel-scrolling');
                    return;
                }
                
                // Calculate next position with easing
                const nextPosition = currentPosition + distance * 0.2;
                carousel.scrollLeft = nextPosition;
                
                // Continue animation
                animationFrameId = requestAnimationFrame(animateScroll);
            }
            
            animationFrameId = requestAnimationFrame(animateScroll);
            
            // Remove active class
            carousel.classList.remove('carousel-scrolling');
        });
    });
    
    // =========================
    // Mobile Navigation Handling
    // =========================
    
    // Make navigation more responsive
    const navLinks = document.querySelectorAll('.nav a.cmd');
    const sections = document.querySelectorAll('section[id]');
    
    // Update active tab based on visible section
    function updateActiveTab() {
        let current = '';
        let maxVisibility = 0;
        
        sections.forEach(section => {
            const rect = section.getBoundingClientRect();
            const sectionHeight = rect.height;
            const visibleHeight = Math.min(rect.bottom, window.innerHeight) - Math.max(rect.top, 0);
            const visibilityRatio = visibleHeight / sectionHeight;
            
            if (visibilityRatio > maxVisibility && visibilityRatio > 0.1) {
                maxVisibility = visibilityRatio;
                current = `#${section.getAttribute('id')}`;
            }
        });
        
        navLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === current) {
                link.classList.add('active');
                
                // Add haptic feedback if supported
                if (window.navigator && window.navigator.vibrate) {
                    window.navigator.vibrate(10);
                }
            }
        });
    }
    
    // Initial check
    updateActiveTab();
    
    // Listen for scroll events with throttling for better performance
    let ticking = false;
    window.addEventListener('scroll', function() {
        if (!ticking) {
            window.requestAnimationFrame(function() {
                updateActiveTab();
                ticking = false;
            });
            ticking = true;
        }
    });
    
    // =========================
    // Floating Action Button
    // =========================
    
    const fab = document.getElementById('mobile-fab');
    if (fab) {
        // Show/hide FAB based on scroll position with smooth transition
        let lastScrollPosition = 0;
        let fabVisible = false;
        
        window.addEventListener('scroll', function() {
            const currentScrollPosition = window.scrollY;
            
            // Show when scrolled down more than 300px
            if (currentScrollPosition > 300 && !fabVisible) {
                fab.style.display = 'flex';
                // Small delay to ensure display has updated
                setTimeout(() => {
                    fab.style.opacity = '1';
                    fab.style.transform = 'scale(1)';
                }, 10);
                fabVisible = true;
            } 
            // Hide when scrolled to top
            else if (currentScrollPosition <= 300 && fabVisible) {
                fab.style.opacity = '0';
                fab.style.transform = 'scale(0.8)';
                setTimeout(() => {
                    if (window.scrollY <= 300) {
                        fab.style.display = 'none';
                    }
                }, 300);
                fabVisible = false;
            }
            
            lastScrollPosition = currentScrollPosition;
        });
        
        // Scroll to top with smooth animation
        fab.addEventListener('click', function() {
            // Add haptic feedback if supported
            if (window.navigator && window.navigator.vibrate) {
                window.navigator.vibrate([15, 10, 15]);
            }
            
            window.scrollTo({
                top: 0,
                behavior: 'smooth'
            });
        });
        
        // Initialize FAB state
        if (window.scrollY > 300) {
            fab.style.display = 'flex';
            fab.style.opacity = '1';
            fabVisible = true;
        } else {
            fab.style.display = 'none';
            fab.style.opacity = '0';
            fabVisible = false;
        }
    }
    
    // =========================
    // Native-like Pull-to-refresh
    // =========================
    
    const pullIndicator = document.getElementById('pull-indicator');
    let touchStartY = 0;
    let isPulling = false;
    let pullDistance = 0;
    let pullThreshold = 80;
    
    document.addEventListener('touchstart', function(e) {
        // Only trigger at the top of the page
        if (window.scrollY <= 5) {
            touchStartY = e.touches[0].clientY;
            isPulling = true;
            pullDistance = 0;
        }
    });
    
    document.addEventListener('touchmove', function(e) {
        if (!isPulling) return;
        
        const touchY = e.touches[0].clientY;
        pullDistance = Math.max(0, touchY - touchStartY);
        
        if (pullDistance > 0 && pullDistance < pullThreshold * 1.5) {
            // Progress animation
            const progress = pullDistance / pullThreshold;
            pullIndicator.style.transform = `scaleX(${progress})`;
            pullIndicator.classList.add('active');
            
            // Add resistance - the further you pull, the harder it gets
            if (progress > 0.8) {
                document.body.style.transform = `translateY(${pullDistance * 0.1}px)`;
            }
            
            // Prevent default scrolling only when actually pulling
            if (window.scrollY === 0 && pullDistance > 5) {
                e.preventDefault();
            }
        }
    }, { passive: false });
    
    document.addEventListener('touchend', function() {
        if (!isPulling) return;
        
        pullIndicator.style.transform = 'scaleX(0)';
        document.body.style.transform = '';
        
        // If pulled enough, trigger refresh action
        if (pullDistance >= pullThreshold) {
            // Visual feedback
            pullIndicator.style.transform = 'scaleX(1)';
            
            // Add haptic feedback if supported
            if (window.navigator && window.navigator.vibrate) {
                window.navigator.vibrate([20, 30, 20]);
            }
            
            // Simulate refresh (in a real app, this would fetch new data)
            setTimeout(() => {
                window.location.reload();
            }, 500);
        } else {
            pullIndicator.classList.remove('active');
        }
        
        isPulling = false;
        pullDistance = 0;
    });
    
    // =========================
    // Swipe Between Sections
    // =========================
    
    // Initialize swipe detection for sections
    const mainElement = document.querySelector('main');
    let touchStartX = 0;
    let touchStartY = 0;
    
    document.addEventListener('touchstart', function(e) {
        touchStartX = e.touches[0].clientX;
        touchStartY = e.touches[0].clientY;
    });
    
    document.addEventListener('touchend', function(e) {
        if (!e.changedTouches[0]) return;
        
        const touchEndX = e.changedTouches[0].clientX;
        const touchEndY = e.changedTouches[0].clientY;
        
        const deltaX = touchEndX - touchStartX;
        const deltaY = touchEndY - touchStartY;
        
        // Only handle horizontal swipes that are significant and more horizontal than vertical
        if (Math.abs(deltaX) > 100 && Math.abs(deltaX) > Math.abs(deltaY)) {
            // Find current active section
            let activeSection = null;
            let activeSectionIndex = -1;
            
            sections.forEach((section, index) => {
                if (section.getBoundingClientRect().top <= 100 && 
                    section.getBoundingClientRect().bottom >= window.innerHeight / 2) {
                    activeSection = section;
                    activeSectionIndex = index;
                }
            });
            
            if (activeSection) {
                // Swipe right to previous section
                if (deltaX > 0 && activeSectionIndex > 0) {
                    // Add haptic feedback
                    if (window.navigator && window.navigator.vibrate) {
                        window.navigator.vibrate(15);
                    }
                    
                    sections[activeSectionIndex - 1].scrollIntoView({
                        behavior: 'smooth'
                    });
                }
                // Swipe left to next section
                else if (deltaX < 0 && activeSectionIndex < sections.length - 1) {
                    // Add haptic feedback
                    if (window.navigator && window.navigator.vibrate) {
                        window.navigator.vibrate(15);
                    }
                    
                    sections[activeSectionIndex + 1].scrollIntoView({
                        behavior: 'smooth'
                    });
                }
            }
        }
    });
});