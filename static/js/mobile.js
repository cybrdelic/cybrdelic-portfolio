// mobile.js - Enhanced mobile-specific JavaScript functionality
// Focuses on touch interactions while maintaining cohesive site experience

document.addEventListener("DOMContentLoaded", function() {
    // Only run on mobile devices or when testing on desktop with mobile width
    const isMobile = window.innerWidth <= 768;
    if (!isMobile) return;
    
    // Add a body class to identify mobile devices for CSS targeting
    document.body.classList.add('mobile-device');
    
    // Create scroll indicators for horizontally scrollable elements
    function createScrollIndicators() {
        // Projects list
        const projectsList = document.querySelector('.projects-list');
        if (projectsList) {
            const projectItems = projectsList.querySelectorAll('.project-list-item');
            if (projectItems.length > 1) {
                // Create indicator container
                const indicator = document.createElement('div');
                indicator.className = 'scroll-indicator';
                
                // Add dots for each project
                projectItems.forEach((_, index) => {
                    const dot = document.createElement('span');
                    dot.className = 'scroll-indicator-dot';
                    if (index === 0) dot.classList.add('active');
                    indicator.appendChild(dot);
                });
                
                // Add after the projects list
                projectsList.parentNode.insertBefore(indicator, projectsList.nextSibling);
                
                // Update active dot on scroll
                projectsList.addEventListener('scroll', function() {
                    requestAnimationFrame(() => {
                        const scrollLeft = this.scrollLeft;
                        const itemWidth = projectItems[0].offsetWidth + 
                                        parseInt(window.getComputedStyle(projectItems[0]).marginRight);
                        const activeIndex = Math.round(scrollLeft / itemWidth);
                        
                        document.querySelectorAll('.scroll-indicator-dot').forEach((dot, i) => {
                            dot.classList.toggle('active', i === activeIndex);
                        });
                    });
                }, { passive: true });
            }
        }
        
        // Career timeline
        const timeline = document.querySelector('#career .timeline');
        if (timeline) {
            const timelineItems = timeline.querySelectorAll('.timeline-item');
            if (timelineItems.length > 1) {
                // Create indicator container
                const indicator = document.createElement('div');
                indicator.className = 'scroll-indicator';
                
                // Add dots for each timeline item
                timelineItems.forEach((_, index) => {
                    const dot = document.createElement('span');
                    dot.className = 'scroll-indicator-dot';
                    if (index === 0) dot.classList.add('active');
                    indicator.appendChild(dot);
                });
                
                // Add after the timeline
                timeline.parentNode.insertBefore(indicator, timeline.nextSibling);
                
                // Update active dot on scroll
                timeline.addEventListener('scroll', function() {
                    requestAnimationFrame(() => {
                        const scrollLeft = this.scrollLeft;
                        const itemWidth = timelineItems[0].offsetWidth + 
                                        parseInt(window.getComputedStyle(timelineItems[0]).marginRight);
                        const activeIndex = Math.round(scrollLeft / itemWidth);
                        
                        document.querySelectorAll('#career .scroll-indicator-dot').forEach((dot, i) => {
                            dot.classList.toggle('active', i === activeIndex);
                        });
                    });
                }, { passive: true });
            }
        }
    }
    
    // Initialize scroll indicators
    createScrollIndicators();
    
    // =========================
    // Subtle Ripple Effect (matches site aesthetic)
    // =========================
    
    // Add ripple effect to all interactive elements - with better performance
    const interactiveSelectors = `
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
    `;
    
    // Use event delegation for better performance
    document.addEventListener('touchstart', function(event) {
        if (event.target.matches && event.target.matches(interactiveSelectors) || 
            event.target.closest && event.target.closest(interactiveSelectors)) {
            const element = event.target.matches(interactiveSelectors) ? 
                            event.target : 
                            event.target.closest(interactiveSelectors);
            
            // Set position relative if needed
            if (window.getComputedStyle(element).position === 'static') {
                element.style.position = 'relative';
            }
            
            createRipple(event, element);
        }
    }, { passive: true });
    
    // Also add mousedown for desktop testing
    document.addEventListener('mousedown', function(event) {
        if (event.target.matches && event.target.matches(interactiveSelectors) || 
            event.target.closest && event.target.closest(interactiveSelectors)) {
            const element = event.target.matches(interactiveSelectors) ? 
                            event.target : 
                            event.target.closest(interactiveSelectors);
            
            // Set position relative if needed
            if (window.getComputedStyle(element).position === 'static') {
                element.style.position = 'relative';
            }
            
            createRipple(event, element);
        }
    });
    
    function createRipple(event, element) {
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
        let startY;
        let startScrollLeft;
        let startTime;
        let targetScrollLeft;
        let animationFrameId;
        let velocity = 0;
        let lastX;
        let lastTime;
        let isScrollingHorizontally = false;
        
        // Handle start of touch
        carousel.addEventListener('touchstart', function(e) {
            // Cancel any ongoing animations
            if (animationFrameId) {
                cancelAnimationFrame(animationFrameId);
            }
            
            isScrolling = true;
            isScrollingHorizontally = false;
            startX = e.touches[0].pageX;
            startY = e.touches[0].pageY;
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
            const y = e.touches[0].pageY;
            const dx = startX - x;
            const dy = startY - y;
            
            // Determine if scrolling horizontally or vertically
            if (!isScrollingHorizontally && Math.abs(dx) > Math.abs(dy) && Math.abs(dx) > 10) {
                isScrollingHorizontally = true;
            }
            
            if (!isScrollingHorizontally) return;
            
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
            if (!isScrolling || !isScrollingHorizontally) {
                carousel.classList.remove('touch-scrolling');
                return;
            }
            
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
                        
                        // Update scroll indicators
                        if (carousel.classList.contains('projects-list')) {
                            document.querySelectorAll('.scroll-indicator-dot').forEach((dot, i) => {
                                dot.classList.toggle('active', i === activeIndex);
                            });
                        } else if (carousel.classList.contains('timeline')) {
                            document.querySelectorAll('#career .scroll-indicator-dot').forEach((dot, i) => {
                                dot.classList.toggle('active', i === activeIndex);
                            });
                            
                            // Trigger timeline item click to update content
                            if (items[activeIndex]) {
                                setTimeout(() => {
                                    items[activeIndex].click();
                                }, 300);
                            }
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
    // Fix for Tech Icons animation
    // =========================
    
    // Ensure tech icons are visible and properly animated on mobile
    function fixTechIcons() {
        const techContainers = document.querySelectorAll('.tech-icons-container');
        
        techContainers.forEach(container => {
            const icons = container.querySelectorAll('.tech-icon');
            if (icons.length === 0) return;
            
            // Create a macro container
            const scrollContainer = container.querySelector('.tech-icons-scroll');
            if (!scrollContainer) return;
            
            // Clear existing content
            scrollContainer.innerHTML = '';
            
            // Create macro element
            const techMacro = document.createElement('div');
            techMacro.className = 'tech-macro';
            
            // Clone icons and add them twice for continuous scrolling
            icons.forEach(icon => {
                const clone = icon.cloneNode(true);
                techMacro.appendChild(clone);
            });
            
            icons.forEach(icon => {
                const clone = icon.cloneNode(true);
                techMacro.appendChild(clone);
            });
            
            // Add to DOM
            scrollContainer.appendChild(techMacro);
            
            // Add touch event listener to pause animation on touch
            techMacro.addEventListener('touchstart', function() {
                this.style.animationPlayState = 'paused';
            }, {passive: true});
            
            techMacro.addEventListener('touchend', function() {
                this.style.animationPlayState = 'running';
            }, {passive: true});
        });
    }
    
    // Run tech icons fix on load and after any dynamic content changes
    fixTechIcons();
    
    // Create MutationObserver to watch for changes in the DOM
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            if (mutation.addedNodes.length) {
                const techContainer = document.querySelector('#career .tech-icons-container');
                if (techContainer && techContainer.closest('.detail-content')) {
                    fixTechIcons();
                }
            }
        });
    });
    
    // Start observing the career detail panel
    const careerDetailPanel = document.querySelector('#career .detail-panel');
    if (careerDetailPanel) {
        observer.observe(careerDetailPanel, { childList: true, subtree: true });
    }
    
    // =========================
    // Mobile Navigation Handling
    // =========================
    
    // Initialize and optimize project/career sections for mobile
    function initMobileSections() {
        // Make project items horizontally scrollable
        const projectsList = document.querySelector('.projects-list');
        if (projectsList) {
            // Show first item by default
            if (projectsList.children.length > 0) {
                const firstItem = projectsList.children[0];
                firstItem.classList.add('active');
                
                // Update project details based on first item
                const dataId = firstItem.getAttribute('data-id');
                if (dataId) {
                    // Extract hidden data for display
                    const title = firstItem.querySelector('.project-list-title')?.textContent || 'Project Name';
                    const subtitle = firstItem.getAttribute('data-subtitle') || 'Project Type';
                    const year = firstItem.getAttribute('data-year') || '2023';
                    const status = firstItem.getAttribute('data-status') || 'Active';
                    const description = firstItem.querySelector('.hidden-description')?.textContent || 'No description available.';
                    const techItems = firstItem.querySelectorAll('.tech-item');
                    
                    // Update the project details panel
                    document.querySelector('.selected-project-title').textContent = title;
                    document.querySelector('.selected-project-subtitle').textContent = subtitle;
                    document.querySelector('.selected-project-description').textContent = description;
                    document.querySelector('.selected-project-status').textContent = status;
                    document.querySelector('.selected-project-year').textContent = year;
                    
                    // Add tech tags
                    const techTagsContainer = document.querySelector('.selected-project-tech-tags');
                    if (techTagsContainer) {
                        techTagsContainer.innerHTML = '';
                        techItems.forEach((item, index) => {
                            const techTag = document.createElement('span');
                            techTag.className = 'tech-tag highlight-on-hover';
                            techTag.textContent = item.textContent;
                            techTagsContainer.appendChild(techTag);
                        });
                    }
                    
                    // Update details link with appropriate URL
                    const detailsLink = document.querySelector('.details-link');
                    if (detailsLink) {
                        const basePath = document.querySelector('script')?.textContent.match(/const base_path = "([^"]+)";/)?.[1] || '';
                        detailsLink.href = `${basePath}projects/${dataId}`;
                    }
                    
                    // Show the project details panel
                    document.querySelector('.default-project-view').style.display = 'none';
                    document.querySelector('.selected-project-view').style.display = 'block';
                }
            }
            
            // Handle click events for mobile
            projectsList.querySelectorAll('.project-list-item').forEach(item => {
                item.addEventListener('click', function(e) {
                    // Don't navigate to project page on card click - let user tap the "View Full Details" link instead
                    e.preventDefault();
                    e.stopPropagation();
                    
                    // Mark this item as active
                    document.querySelectorAll('.project-list-item').forEach(i => i.classList.remove('active'));
                    this.classList.add('active');
                    
                    // Update scroll indicator
                    const items = Array.from(projectsList.children);
                    const index = items.indexOf(this);
                    document.querySelectorAll('.scroll-indicator-dot').forEach((dot, i) => {
                        dot.classList.toggle('active', i === index);
                    });
                    
                    // Extract hidden data for display
                    const dataId = this.getAttribute('data-id');
                    const title = this.querySelector('.project-list-title')?.textContent || 'Project Name';
                    const subtitle = this.getAttribute('data-subtitle') || 'Project Type';
                    const year = this.getAttribute('data-year') || '2023';
                    const status = this.getAttribute('data-status') || 'Active';
                    const description = this.querySelector('.hidden-description')?.textContent || 'No description available.';
                    const techItems = this.querySelectorAll('.tech-item');
                    
                    // Add a subtle animation to the title
                    const titleElement = document.querySelector('.selected-project-title');
                    if (titleElement && titleElement.textContent !== title) {
                        titleElement.style.opacity = '0';
                        setTimeout(() => {
                            titleElement.textContent = title;
                            titleElement.style.opacity = '1';
                        }, 200);
                    } else if (titleElement) {
                        titleElement.textContent = title;
                    }
                    
                    // Update other details with subtle animations
                    const subtitle_el = document.querySelector('.selected-project-subtitle');
                    const description_el = document.querySelector('.selected-project-description');
                    const status_el = document.querySelector('.selected-project-status');
                    const year_el = document.querySelector('.selected-project-year');
                    
                    if (subtitle_el) subtitle_el.textContent = subtitle;
                    if (description_el) {
                        description_el.style.opacity = '0';
                        setTimeout(() => {
                            description_el.textContent = description;
                            description_el.style.opacity = '1';
                        }, 150);
                    }
                    if (status_el) status_el.textContent = status;
                    if (year_el) year_el.textContent = year;
                    
                    // Update tech tags with staggered animation
                    const techTagsContainer = document.querySelector('.selected-project-tech-tags');
                    if (techTagsContainer) {
                        techTagsContainer.innerHTML = '';
                        techItems.forEach((item, index) => {
                            const techTag = document.createElement('span');
                            techTag.className = 'tech-tag highlight-on-hover';
                            techTag.textContent = item.textContent;
                            techTag.style.opacity = '0';
                            techTag.style.transform = 'translateY(10px)';
                            techTagsContainer.appendChild(techTag);
                            
                            setTimeout(() => {
                                techTag.style.transition = 'all 0.3s ease';
                                techTag.style.opacity = '1';
                                techTag.style.transform = 'translateY(0)';
                            }, 200 + (index * 50));
                        });
                    }
                    
                    // Update details link
                    const detailsLink = document.querySelector('.details-link');
                    if (detailsLink) {
                        const basePath = document.querySelector('script')?.textContent.match(/const base_path = "([^"]+)";/)?.[1] || '';
                        detailsLink.href = `${basePath}projects/${dataId}`;
                    }
                    
                    // Show the selected view
                    document.querySelector('.default-project-view').style.display = 'none';
                    document.querySelector('.selected-project-view').style.display = 'block';
                    
                    // Scroll to details panel
                    const detailsPanel = document.querySelector('.project-details-panel');
                    if (detailsPanel) {
                        // Apply a subtle animation to the panel
                        detailsPanel.style.transform = 'translateY(5px)';
                        detailsPanel.style.opacity = '0.95';
                        
                        setTimeout(() => {
                            detailsPanel.scrollIntoView({ 
                                behavior: 'smooth',
                                block: 'start'
                            });
                            
                            // Reset panel animation
                            setTimeout(() => {
                                detailsPanel.style.transform = 'translateY(0)';
                                detailsPanel.style.opacity = '1';
                            }, 200);
                        }, 300);
                    }
                    
                    // Add haptic feedback on selection
                    if (window.navigator && window.navigator.vibrate) {
                        window.navigator.vibrate(10);
                    }
                });
            });
        }
        
        // Career timeline needs better integration with existing handler
        const timelineItems = document.querySelectorAll('#career .timeline-item');
        timelineItems.forEach(item => {
            // Modify original handler to ensure smooth scrolling to details panel
            item.addEventListener('click', function() {
                // After existing handler runs, also smooth scroll to detail panel
                setTimeout(() => {
                    const detailPanel = document.querySelector('#career .detail-panel');
                    if (detailPanel) {
                        detailPanel.scrollIntoView({ 
                            behavior: 'smooth',
                            block: 'start'
                        });
                    }
                    
                    // Add haptic feedback
                    if (window.navigator && window.navigator.vibrate) {
                        window.navigator.vibrate(10);
                    }
                }, 100);
            });
        });
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
    // Mobile Bottom Navigation
    // =========================
    
    // Create and add mobile bottom navigation
    function createMobileBottomNav() {
        // Don't add if it already exists
        if (document.querySelector('.mobile-bottom-nav')) return;
        
        // Create navigation container
        const nav = document.createElement('nav');
        nav.className = 'mobile-bottom-nav';
        
        // Define main sections
        const sections = [
            { id: 'hero', label: 'Home', icon: '🏠' },
            { id: 'projects', label: 'Projects', icon: '🛠️' },
            { id: 'career', label: 'Career', icon: '📈' },
            { id: 'contact', label: 'Contact', icon: '📧' }
        ];
        
        // Create navigation items
        sections.forEach(section => {
            const navItem = document.createElement('a');
            navItem.href = `#${section.id}`;
            navItem.className = 'mobile-nav-item';
            navItem.dataset.section = section.id;
            
            // Add icon
            const icon = document.createElement('span');
            icon.className = 'mobile-nav-icon';
            icon.textContent = section.icon;
            
            // Add label
            const label = document.createElement('span');
            label.className = 'mobile-nav-label';
            label.textContent = section.label;
            
            // Combine elements
            navItem.appendChild(icon);
            navItem.appendChild(label);
            
            // Add click handler
            navItem.addEventListener('click', function(e) {
                e.preventDefault();
                
                // Find section
                const targetSection = document.getElementById(section.id);
                if (targetSection) {
                    // Scroll to section
                    targetSection.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                    
                    // Update active state
                    document.querySelectorAll('.mobile-nav-item').forEach(item => {
                        item.classList.remove('active');
                    });
                    this.classList.add('active');
                    
                    // Add haptic feedback
                    if (window.navigator && window.navigator.vibrate) {
                        window.navigator.vibrate(10);
                    }
                }
            });
            
            // Add to navigation
            nav.appendChild(navItem);
        });
        
        // Add navigation to document
        document.body.appendChild(nav);
        
        // Set initial active item based on scroll position
        updateMobileNav();
    }
    
    // Update active navigation item based on scroll position
    function updateMobileNav() {
        const sections = document.querySelectorAll('section[id]');
        const navItems = document.querySelectorAll('.mobile-nav-item');
        
        // Find the current section
        let currentSectionId = '';
        let maxVisibility = 0;
        
        sections.forEach(section => {
            const rect = section.getBoundingClientRect();
            const sectionHeight = rect.height;
            const visibleHeight = Math.min(rect.bottom, window.innerHeight) - Math.max(rect.top, 0);
            const visibilityRatio = visibleHeight / sectionHeight;
            
            if (visibilityRatio > maxVisibility && visibilityRatio > 0.1) {
                maxVisibility = visibilityRatio;
                currentSectionId = section.id;
            }
        });
        
        // Update active state
        if (currentSectionId) {
            navItems.forEach(item => {
                const isActive = item.dataset.section === currentSectionId;
                item.classList.toggle('active', isActive);
            });
        }
    }
    
    // Add scroll listener to update active navigation item
    window.addEventListener('scroll', function() {
        requestAnimationFrame(updateMobileNav);
    }, { passive: true });
    
    // Initialize mobile navigation
    createMobileBottomNav();
    
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