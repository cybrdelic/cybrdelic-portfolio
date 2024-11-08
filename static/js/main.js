document.addEventListener("DOMContentLoaded", () => {
  initPageLoader();
  initHeroEffects();
  initGlitchEffects();
  initProjectCardEffects();
  initTimestampUpdates();
  initHtmxHandlers();
  initScrollIndicator();
});

// Page Loader Initialization
function initPageLoader() {
  const loader = document.getElementById('page-loader');
  if (loader) {
    window.addEventListener('load', () => {
      loader.classList.add('fade-out');
      setTimeout(() => {
        loader.style.display = 'none';
      }, 500); // Match with CSS transition duration
    });
  }
}

// Hero Section
function initHeroEffects() {
  initStatusLog();
  updateUptime();
  initMatrixPanels();
}

function initStatusLog() {
  const log = document.getElementById('status-log');
  if (!log) return;

  const messages = [
    'initializing systems...',
    'loading neural networks...',
    'establishing grid connection...',
    'scanning network perimeter...',
    'analyzing system vulnerabilities...',
    'updating security protocols...',
    'optimizing neural pathways...',
    'synchronizing quantum matrices...',
    'systems operational'
  ];

  let index = 0;

  function addLogMessage() {
    if (index < messages.length) {
      log.innerHTML += `\n> ${messages[index]}`;
      log.scrollTop = log.scrollHeight;
      index++;

      // Randomize the delay between messages
      const delay = 1000 + Math.random() * 1500;
      setTimeout(addLogMessage, delay);
    }
  }

  // Start the log messages
  setTimeout(addLogMessage, 500);
}

function updateUptime() {
  const uptimeElement = document.getElementById('uptime');
  if (!uptimeElement) return;

  const start = new Date();

  setInterval(() => {
    const now = new Date();
    const diff = now - start;
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);

    uptimeElement.textContent = `${String(hours).padStart(2, '0')}:${String(minutes % 60).padStart(2, '0')}:${String(seconds % 60).padStart(2, '0')}`;
  }, 1000);
}

function initMatrixPanels() {
  // Animate the expertise progress bars
  const progressBars = document.querySelectorAll('.progress-fill');
  progressBars.forEach(bar => {
    const targetWidth = bar.style.width;
    bar.style.width = '0';
    setTimeout(() => {
      bar.style.width = targetWidth;
    }, 500);
  });
}

// Glitch Effects
function initGlitchEffects() {
  const glitchTexts = document.querySelectorAll(".glitch-text");

  glitchTexts.forEach((text) => {
    text.setAttribute("data-text", text.textContent);

    setInterval(() => {
      if (Math.random() > 0.95) {
        text.style.textShadow = `
                    ${Math.random() * 3}px 0 var(--color-primary),
                    ${-Math.random() * 3}px 0 var(--color-secondary)
                `;

        setTimeout(() => {
          text.style.textShadow = "";
        }, 50);
      }
    }, 100);
  });

  // Random glitch effect on panels
  const panels = document.querySelectorAll('.matrix-panel, .identity-panel');
  panels.forEach(panel => {
    setInterval(() => {
      if (Math.random() > 0.98) {
        panel.style.transform = `translateX(${Math.random() * 2}px)`;
        setTimeout(() => {
          panel.style.transform = "none";
        }, 50);
      }
    }, 50);
  });
}

// Project Cards
function initProjectCardEffects() {
  const projectCards = document.querySelectorAll(".project-card");

  projectCards.forEach(card => {
    card.addEventListener("mouseover", () => {
      const header = card.querySelector(".terminal-header");
      if (header) {
        header.style.background = "rgba(0, 255, 159, 0.2)";
      }
      card.style.transform = "translateY(-8px)";
      card.style.boxShadow = "0 12px 24px rgba(0, 255, 159, 0.2)";
    });

    card.addEventListener("mouseout", () => {
      const header = card.querySelector(".terminal-header");
      if (header) {
        header.style.background = "";
      }
      card.style.transform = "";
      card.style.boxShadow = "";
    });
  });
}

// Timestamp Updates
function initTimestampUpdates() {
  function updateTimestamp() {
    const timestamp = document.getElementById("timestamp");
    if (timestamp) {
      const now = new Date();
      const formatted = now.toISOString().replace("T", " ").substring(0, 19);
      timestamp.textContent = formatted;

      if (Math.random() > 0.95) {
        timestamp.style.color = "var(--color-primary)";
        setTimeout(() => {
          timestamp.style.color = "";
        }, 50);
      }
    }
  }

  setInterval(updateTimestamp, 1000);
  updateTimestamp();
}


function initScrollIndicator() {
  const projectsContent = document.querySelector('.projects-content');
  const scrollIndicator = document.createElement('div');
  scrollIndicator.className = 'scroll-indicator';
  scrollIndicator.innerHTML = `
    <span class="scroll-indicator-text">Scroll</span>
    <div class="scroll-indicator-arrow"></div>
  `;

  document.querySelector('.hero-projects').appendChild(scrollIndicator);

  // Hide scroll indicator when user has scrolled
  projectsContent.addEventListener('scroll', () => {
    if (projectsContent.scrollTop > 50) {
      scrollIndicator.classList.add('hidden');
    } else {
      scrollIndicator.classList.remove('hidden');
    }
  });

  // Hide scroll indicator if content doesn't overflow
  function checkOverflow() {
    if (projectsContent.scrollHeight <= projectsContent.clientHeight) {
      scrollIndicator.classList.add('hidden');
    } else {
      scrollIndicator.classList.remove('hidden');
    }
  }

  // Check on load and resize
  window.addEventListener('load', checkOverflow);
  window.addEventListener('resize', checkOverflow);

  // Check when projects are loaded via HTMX
  document.addEventListener('htmx:afterSwap', (event) => {
    if (event.detail.target.classList.contains('project-grid')) {
      checkOverflow();
    }
  });
}


// HTMX Handlers
function initHtmxHandlers() {
  // Loading states
  document.addEventListener('htmx:beforeRequest', function (event) {
    const target = event.detail.target;

    // Project grid loading state
    if (target.classList.contains('project-grid')) {
      target.innerHTML = `
                <div class="project-skeleton">
                    <div class="skeleton skeleton-title"></div>
                    <div class="skeleton skeleton-text"></div>
                </div>
                <div class="project-skeleton">
                    <div class="skeleton skeleton-title"></div>
                    <div class="skeleton skeleton-text"></div>
                </div>
                <div class="project-skeleton">
                    <div class="skeleton skeleton-title"></div>
                    <div class="skeleton skeleton-text"></div>
                </div>
                <!-- Add more skeletons as needed -->
            `;
    }

    // Form submission loading state
    if (target.tagName === 'FORM') {
      const submitButton = target.querySelector('button[type="submit"]');
      if (submitButton) {
        submitButton.disabled = true;
        submitButton.innerHTML = 'transmitting...';
      }
    }
  });

  // Reset form button after request
  document.addEventListener('htmx:afterRequest', function (event) {
    const form = event.detail.elt;
    if (form.tagName === 'FORM') {
      const submitButton = form.querySelector('button[type="submit"]');
      if (submitButton) {
        submitButton.disabled = false;
        submitButton.innerHTML = 'transmit_message()';
      }
    }
  });

  // Add fade-in effect after content is loaded
  document.addEventListener('htmx:afterSwap', function (event) {
    const target = event.detail.target;

    // Apply fade-in to project cards
    if (target.classList.contains('project-grid')) {
      const projectCards = target.querySelectorAll('.project-card');
      projectCards.forEach(card => {
        card.classList.add('fade-in');
      });
    }

    // Apply fade-in to other dynamic content if needed
  });
}

// Smooth scrolling for anchor links
document.addEventListener('click', function (event) {
  if (event.target.tagName === 'A') {
    const href = event.target.getAttribute('href');
    if (href && href.startsWith('#')) {
      event.preventDefault();
      const target = document.querySelector(href);
      if (target) {
        target.scrollIntoView({
          behavior: 'smooth',
          block: 'start'
        });
      }
    }
  }
});

// Utility function for random glitch intervals
function randomGlitch(element, callback, probability = 0.95) {
  if (Math.random() > probability) {
    callback(element);
    setTimeout(() => {
      callback(element);
    }, 50 + Math.random() * 100);
  }
}
