// Update timestamp in footer
function updateTimestamp() {
    const timestamp = document.getElementById('timestamp');
    timestamp.textContent = new Date().toISOString();
}

setInterval(updateTimestamp, 1000);

// Terminal typing animation
document.addEventListener('DOMContentLoaded', () => {
    const typeAnimation = document.querySelector('.type-animation');
    const text = typeAnimation.innerHTML;
    typeAnimation.innerHTML = '';
    
    let i = 0;
    const typeSpeed = 50;

    function typeWriter() {
        if (i < text.length) {
            typeAnimation.innerHTML += text.charAt(i);
            i++;
            setTimeout(typeWriter, typeSpeed);
        }
    }

    typeWriter();
});

// Handle project display
document.addEventListener('htmx:afterSwap', function(evt) {
    if (evt.target.matches('.project-grid')) {
        const projects = JSON.parse(evt.detail.response);
        const projectGrid = evt.target;
        
        projectGrid.innerHTML = projects.map(project => `
            <div class="project-card">
                <h3>${project.title}</h3>
                <p>${project.description}</p>
                <div class="project-tech">
                    ${project.technologies.map(tech => 
                        `<span class="tech-tag">${tech}</span>`
                    ).join('')}
                </div>
                <a href="${project.github_url}" class="cyber-button" target="_blank">
                    View on GitHub
                </a>
            </div>
        `).join('');
    }
});
