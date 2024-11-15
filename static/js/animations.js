// animations.js
const observerOptions = {
    root: null,
    rootMargin: '0px',
    threshold: 0.1
};

// Slide and fade animations for different elements
const animateOnScroll = (entries, observer) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('animate-in');
            observer.unobserve(entry.target); // Only animate once
        }
    });
};

// Create observer
const observer = new IntersectionObserver(animateOnScroll, observerOptions);

// Observe all elements with animation classes
document.addEventListener('DOMContentLoaded', () => {
    const animatedElements = document.querySelectorAll('.fade-in, .slide-up, .slide-in');
    animatedElements.forEach(el => observer.observe(el));
});
