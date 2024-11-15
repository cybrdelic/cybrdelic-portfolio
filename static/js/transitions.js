document.addEventListener('DOMContentLoaded', () => {
    const overlay = document.getElementById('transition-overlay');
    const content = document.getElementById('content');
    
    async function handleNavigation(event) {
        const link = event.target.closest('[data-transition]');
        if (!link) return;
        
        event.preventDefault();
        const url = link.href;
        
        // Start transition
        overlay.classList.add('active');
        await new Promise(r => setTimeout(r, 300));
        
        try {
            // Fetch new content
            const response = await fetch(url);
            const html = await response.text();
            
            // Parse new content
            const parser = new DOMParser();
            const doc = parser.parseFromString(html, 'text/html');
            const newContent = doc.querySelector('#content').innerHTML;
            
            // Update URL and content
            window.history.pushState({}, '', url);
            content.innerHTML = newContent;
            
            // Update page class
            const newPageClass = doc.querySelector('body').className;
            document.body.className = newPageClass;
            
            // Initialize any scripts for new content
            initializePageScripts();
        } catch (error) {
            console.error('Navigation error:', error);
        }
        
        // End transition
        overlay.classList.remove('active');
    }
    
    // Handle navigation events
    document.addEventListener('click', handleNavigation);
    
    // Handle browser back/forward buttons
    window.addEventListener('popstate', async () => {
        overlay.classList.add('active');
        await new Promise(r => setTimeout(r, 300));
        
        const response = await fetch(window.location.href);
        const html = await response.text();
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, 'text/html');
        
        content.innerHTML = doc.querySelector('#content').innerHTML;
        document.body.className = doc.querySelector('body').className;
        
        initializePageScripts();
        overlay.classList.remove('active');
    });
});

function initializePageScripts() {
    // Add any page-specific initialization here
    const projects = document.querySelectorAll('.project');
    projects.forEach(project => {
        project.addEventListener('mouseenter', () => {
            if (Math.random() < 0.3) { // 30% chance of glitch effect
                project.classList.add('glitch');
                setTimeout(() => project.classList.remove('glitch'), 200);
            }
        });
    });
}

// Initialize scripts on first load
initializePageScripts();
