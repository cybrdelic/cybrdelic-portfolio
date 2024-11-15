document.addEventListener('DOMContentLoaded', () => {
    const noise = document.getElementById('noise');
    
    function createNoise() {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        
        canvas.width = 100;
        canvas.height = 100;
        
        const imageData = ctx.createImageData(100, 100);
        const buffer = new Uint32Array(imageData.data.buffer);
        
        for (let i = 0; i < buffer.length; i++) {
            buffer[i] = Math.random() < 0.5 ? 0xFF000000 : 0xFFFFFFFF;
        }
        
        ctx.putImageData(imageData, 0, 0);
        noise.style.backgroundImage = `url(${canvas.toDataURL()})`;
    }
    
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

    setInterval(createNoise, 50);
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
