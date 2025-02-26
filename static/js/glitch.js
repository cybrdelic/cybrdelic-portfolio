document.addEventListener('DOMContentLoaded', () => {


    function glitchEffect() {
        const elements = document.querySelectorAll('h1, h2, .prompt');
        elements.forEach(el => {
            if (Math.random() < 0.1) {
                el.style.animation = 'glitch 0.2s infinite';
                setTimeout(() => {
                    el.style.animation = '';
                }, 200);
            }
        });
    }

    setInterval(glitchEffect, 3000);

    // Add terminal typing effect to project descriptions
    const descriptions = document.querySelectorAll('.project p');
    descriptions.forEach(desc => {
        const text = desc.textContent;
        desc.textContent = '';
        let i = 0;
        const speed = 20;

        function typeWriter() {
            if (i < text.length) {
                desc.textContent += text.charAt(i);
                i++;
                setTimeout(typeWriter, speed);
            }
        }

        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    typeWriter();
                    observer.unobserve(entry.target);
                }
            });
        });

        observer.observe(desc);
    });
});
