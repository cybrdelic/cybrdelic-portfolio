// Add this to a new file called documentation.js in your static/js directory

document.addEventListener('DOMContentLoaded', () => {
    enhanceDocumentation();
});

function enhanceDocumentation() {
    // Add anchor links to headings
    addHeadingAnchors();

    // Add active state to TOC based on scroll position
    setupScrollSpy();

    // Generate section wrappers
    createSectionWrappers();

    // Setup click handlers for TOC items
    setupTocNavigation();
}

function addHeadingAnchors() {
    const headings = document.querySelectorAll('.markdown-heading');

    headings.forEach(heading => {
        const id = heading.getAttribute('id');
        if (!id) return;

        // Create anchor element
        const anchor = document.createElement('a');
        anchor.classList.add('heading-anchor');
        anchor.href = `#${id}`;
        anchor.innerHTML = '&para;';
        anchor.title = 'Link to this section';

        heading.appendChild(anchor);
    });
}

function setupScrollSpy() {
    const headings = Array.from(document.querySelectorAll('.markdown-heading'));
    const tocLinks = document.querySelectorAll('.table-of-contents a');

    if (!headings.length || !tocLinks.length) return;

    // Debounce scroll event
    let scrollTimeout;

    window.addEventListener('scroll', () => {
        if (scrollTimeout) {
            window.cancelAnimationFrame(scrollTimeout);
        }

        scrollTimeout = window.requestAnimationFrame(() => {
            // Find the current heading in view
            const currentHeading = headings.find(heading => {
                const rect = heading.getBoundingClientRect();
                const headerOffset = 100; // Adjust based on your fixed header height
                return rect.top <= headerOffset && rect.bottom > headerOffset;
            });

            if (currentHeading) {
                // Get the ID
                const currentId = currentHeading.getAttribute('id');

                // Remove active class from all TOC links
                tocLinks.forEach(link => {
                    link.classList.remove('active');
                });

                // Add active class to the corresponding TOC link
                const activeLink = document.querySelector(`.table-of-contents a[href="#${currentId}"]`);
                if (activeLink) {
                    activeLink.classList.add('active');
                }
            }
        });
    });
}

function createSectionWrappers() {
    const content = document.querySelector('.documentation-wrapper');
    if (!content) return;

    // Get all headings
    const headings = Array.from(content.querySelectorAll('.markdown-heading'));

    headings.forEach((heading, index) => {
        // Create a section wrapper
        const section = document.createElement('div');
        section.classList.add('markdown-section');

        // Get all elements until the next heading or end
        const elements = [];
        let nextElement = heading.nextElementSibling;

        while (nextElement && !nextElement.classList.contains('markdown-heading')) {
            elements.push(nextElement);
            nextElement = nextElement.nextElementSibling;
        }

        // Move heading and all related elements to the section
        section.appendChild(heading.cloneNode(true));
        elements.forEach(el => {
            section.appendChild(el.cloneNode(true));
        });

        // Replace the original content with the new section
        if (index === 0) {
            content.innerHTML = '';
            content.appendChild(section);
        } else {
            content.appendChild(section);
        }
    });
}

function setupTocNavigation() {
    const tocLinks = document.querySelectorAll('.table-of-contents a');

    tocLinks.forEach(link => {
        link.addEventListener('click', (event) => {
            event.preventDefault();

            const targetId = link.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);

            if (targetElement) {
                // Remove active class from all links
                tocLinks.forEach(l => l.classList.remove('active'));

                // Add active class to clicked link
                link.classList.add('active');

                // Smooth scroll to target
                window.scrollTo({
                    top: targetElement.offsetTop - 80, // Adjust based on your header height
                    behavior: 'smooth'
                });
            }
        });
    });
}
