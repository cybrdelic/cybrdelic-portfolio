// mobile.js - Enhanced mobile-specific JavaScript functionality
// Focuses on touch interactions while maintaining cohesive site experience

document.addEventListener("DOMContentLoaded", function() {
    // Only run on mobile devices or when testing on desktop with mobile width
    const isMobile = window.innerWidth <= 768;
    if (!isMobile) return;
    
    // =========================
    // Subtle Ripple Effect (matches site aesthetic)
    // =========================
    
    // Add ripple effect to all interactive elements
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
        button,
        a[href]:not(.nav-left a):not(.nav-right a),
        .project-meta-item
    `);
    
    interactiveElements.forEach(el => {
        // Set position relative if needed
        if (window.getComputedStyle(el).position === 'static') {
            el.style.position = 'relative';
        }
        
        // Add ripple effect
        el.addEventListener('touchstart', createRipple, {passive: true});
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
        
        // Calculate size (slightly smaller for subtler effect matching site design)
        const size = Math.max(rect.width, rect.height) * 1.8;
        
        // Position and size ripple
        ripple.style.width = ripple.style.height = `${size}px`;
        ripple.style.left = `${x - size/2}px`;
        ripple.style.top = `${y - size/2}px`;
        
        // Add haptic feedback if supported (more subtle)
        if (window.navigator && window.navigator.vibrate) {
            window.navigator.vibrate(5); // Shorter, more subtle vibration
        }
        
        // Clean up ripple after animation completes
        setTimeout(() => {
            if (ripple.parentNode === element) {
                element.removeChild(ripple);
            }
        }, 600);
    }
    
    // =========================
    // Enhanced Scroll Physics for Carousels
    // =========================
    
    // Find all horizontal carousels
    const carousels = document.querySelectorAll('.projects-list, #career .timeline, .selected-project-tech-tags, .tech-icons-container, .expertise-tags, .project-tech, .topic-tags, #career .topic-tags');
    
    carousels.forEach(carousel => {
        let isScrolling = false;
        let startX;
        let startScrollLeft;
        let startTime;
        let targetScrollLeft;
        let animationFrameId;
        let velocity = 0;
        let lastX;
        let lastTime;
        
        // Handle start of touch
        carousel.addEventListener('touchstart', function(e) {
            // Cancel any ongoing animations
            if (animationFrameId) {
                cancelAnimationFrame(animationFrameId);
            }
            
            isScrolling = true;
            startX = e.touches[0].pageX;
            lastX = startX;
            startScrollLeft = carousel.scrollLeft;
            startTime = Date.now();
            lastTime = startTime;
            velocity = 0;
            
            // Add active class to indicate scrolling
            carousel.classList.add('touch-scrolling');
        }, {passive: true});
        
        // Handle touchmove
        carousel.addEventListener('touchmove', function(e) {
            if (!isScrolling) return;
            
            // Calculate how far finger has moved
            const x = e.touches[0].pageX;
            const dx = startX - x;
            
            // Calculate instantaneous velocity
            const currentTime = Date.now();
            const dt = currentTime - lastTime;
            if (dt > 0) {
                velocity = (lastX - x) / dt;
            }
            
            // Update last positions
            lastX = x;
            lastTime = currentTime;
            
            // Update scroll position
            carousel.scrollLeft = startScrollLeft + dx;
        }, {passive: true});
        
        // Handle end of touch - with momentum and snap
        carousel.addEventListener('touchend', function(e) {
            if (!isScrolling) return;
            
            isScrolling = false;
            const endScrollLeft = carousel.scrollLeft;
            
            // Only add momentum if velocity is significant
            if (Math.abs(velocity) > 0.1) {
                // Calculate target position with momentum
                const momentum = velocity * 300; // Adjust for stronger/weaker momentum
                targetScrollLeft = endScrollLeft + momentum;
                
                // Find nearest snap point (if element has children that are cards)
                if (carousel.children.length > 0 && 
                    (carousel.classList.contains('projects-list') || 
                     carousel.classList.contains('timeline'))) {
                    
                    // For projects and career timelines, use snap points
                    const cardWidth = carousel.children[0].offsetWidth + 
                                    parseInt(window.getComputedStyle(carousel.children[0]).marginRight || 0);
                    const snapPoint = Math.round(targetScrollLeft / cardWidth) * cardWidth;
                    targetScrollLeft = snapPoint;
                    
                    // Add active class to the selected item
                    if (carousel.classList.contains('projects-list') || carousel.classList.contains('timeline')) {
                        const activeIndex = Math.round(targetScrollLeft / cardWidth);
                        const items = carousel.children;
                        for (let i = 0; i < items.length; i++) {
                            items[i].classList.toggle('active', i === activeIndex);
                        }
                    }
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
                        carousel.classList.remove('touch-scrolling');
                        return;
                    }
                    
                    // Calculate next position with easing
                    const nextPosition = currentPosition + distance * 0.15;
                    carousel.scrollLeft = nextPosition;
                    
                    // Continue animation
                    animationFrameId = requestAnimationFrame(animateScroll);
                }
                
                animationFrameId = requestAnimationFrame(animateScroll);
            } else {
                carousel.classList.remove('touch-scrolling');
            }
        }, {passive: true});
    });
    
    // =========================
    // Mobile Navigation Handling
    // =========================
    
    // Initialize project/career sections
    function initMobileSections() {
        // Make project items horizontally scrollable
        const projectsList = document.querySelector('.projects-list');
        if (projectsList) {
            // Show first item by default
            if (projectsList.children.length > 0) {
                projectsList.children[0].classList.add('active');
                const projectId = projectsList.children[0].getAttribute('data-project-id');
                if (projectId) {
                    document.querySelectorAll('.selected-project-view').forEach(view => {
                        view.style.display = view.getAttribute('data-project-id') === projectId ? 'block' : 'none';
                    });
                }
            }
            
            // Handle click events for mobile
            projectsList.querySelectorAll('.project-list-item').forEach(item => {
                item.addEventListener('click', function(e) {
                    const projectId = this.getAttribute('data-project-id');
                    if (projectId) {
                        document.querySelectorAll('.project-list-item').forEach(i => i.classList.remove('active'));
                        this.classList.add('active');
                        
                        document.querySelectorAll('.selected-project-view').forEach(view => {
                            view.style.display = view.getAttribute('data-project-id') === projectId ? 'block' : 'none';
                        });
                        
                        // Scroll to details panel
                        const detailsPanel = document.querySelector('.project-details-panel');
                        if (detailsPanel) {
                            setTimeout(() => {
                                detailsPanel.scrollIntoView({ behavior: 'smooth' });
                            }, 300);
                        }
                    }
                });
            });
        }
        
        // Handle career timeline
        const timeline = document.querySelector('#career .timeline');
        if (timeline) {
            // Show first item by default
            if (timeline.children.length > 0) {
                timeline.children[0].classList.add('active');
                const jobId = timeline.children[0].getAttribute('data-job-id');
                if (jobId) {
                    document.querySelectorAll('.job-detail').forEach(job => {
                        job.style.display = job.getAttribute('data-job-id') === jobId ? 'block' : 'none';
                    });
                }
            }
            
            // Handle click events for mobile
            timeline.querySelectorAll('.timeline-item').forEach(item => {
                item.addEventListener('click', function(e) {
                    const jobId = this.getAttribute('data-job-id');
                    if (jobId) {
                        document.querySelectorAll('.timeline-item').forEach(i => i.classList.remove('active'));
                        this.classList.add('active');
                        
                        document.querySelectorAll('.job-detail').forEach(job => {
                            job.style.display = job.getAttribute('data-job-id') === jobId ? 'block' : 'none';
                        });
                        
                        // Scroll to details panel
                        const detailPanel = document.querySelector('#career .detail-panel');
                        if (detailPanel) {
                            setTimeout(() => {
                                detailPanel.scrollIntoView({ behavior: 'smooth' });
                            }, 300);
                        }
                    }
                });
            });
        }
    }
    
    // Initialize mobile-specific enhancements
    initMobileSections();
    
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
            const wasActive = link.classList.contains('active');
            const isActive = link.getAttribute('href') === current;
            
            link.classList.toggle('active', isActive);
            
            // Add haptic feedback when section changes (only for newly active items)
            if (!wasActive && isActive && window.navigator && window.navigator.vibrate) {
                window.navigator.vibrate(8); // More subtle vibration
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
    }, {passive: true});
    
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
        }, {passive: true});
        
        // Scroll to top with smooth animation
        fab.addEventListener('click', function() {
            // Add subtle haptic feedback if supported
            if (window.navigator && window.navigator.vibrate) {
                window.navigator.vibrate([10, 5, 10]); // More subtle pattern
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
    }, {passive: true});
    
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
        }
    }, {passive: true});
    
    document.addEventListener('touchend', function() {
        if (!isPulling) return;
        
        pullIndicator.style.transform = 'scaleX(0)';
        document.body.style.transform = '';
        
        // If pulled enough, trigger refresh action
        if (pullDistance >= pullThreshold) {
            // Visual feedback
            pullIndicator.style.transform = 'scaleX(1)';
            
            // Add subtle haptic feedback if supported
            if (window.navigator && window.navigator.vibrate) {
                window.navigator.vibrate([15, 20, 15]); // More subtle pattern
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
    // Enhanced Touch Feedback
    // =========================
    
    // Apply active state on touch for interactive elements
    const touchElements = document.querySelectorAll(`
        .cmd, 
        .project-list-item,
        .project-card,
        .tech-tag,
        .details-link,
        #career .timeline-item,
        .project-tech-item,
        .expertise-tag,
        .topic-card,
        button,
        a[href]:not(.nav-left a):not(.nav-right a)
    `);
    
    touchElements.forEach(element => {
        element.addEventListener('touchstart', function() {
            this.classList.add('touch-active');
        }, {passive: true});
        
        element.addEventListener('touchend', function() {
            this.classList.remove('touch-active');
        }, {passive: true});
        
        element.addEventListener('touchcancel', function() {
            this.classList.remove('touch-active');
        }, {passive: true});
    });
    
    // Disable hover states for touch devices
    document.documentElement.classList.add('touch-device');
});