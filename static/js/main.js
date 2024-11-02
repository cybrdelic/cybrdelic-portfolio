document.addEventListener("DOMContentLoaded", () => {
  initTerminalTyping();
  initGlitchEffects();
  initCircuitAnimations();
  initProjectCardEffects();
  initTimestampUpdates();
  initTerminalEffects();
});

function initTerminalTyping() {
  const terminalText = document.getElementById('terminal-text');
  if (!terminalText) return;

  const commands = [
    { command: 'whoami', response: 'Alejandro Figueroa' },
    { command: 'cat description.txt', response: 'Full-stack developer specializing in autonomous systems and AI integration.' }
  ];

  let currentLine = 0;
  let currentChar = 0;
  let isTypingCommand = true;
  let currentText = '';

  function typeText() {
    if (currentLine >= commands.length) return;

    const text = isTypingCommand
      ? `<span class="prompt">$</span> ${commands[currentLine].command}`
      : `<span class="response">${commands[currentLine].response}</span>`;

    if (currentChar < text.length) {
      currentText += text[currentChar];
      terminalText.innerHTML = currentText + '\n';
      currentChar++;
      setTimeout(typeText, 50);
    } else {
      currentText += '\n';
      if (isTypingCommand) {
        isTypingCommand = false;
        currentChar = 0;
        setTimeout(typeText, 500);
      } else {
        isTypingCommand = true;
        currentChar = 0;
        currentLine++;
        setTimeout(typeText, 1000);
      }
    }
  }

  setTimeout(typeText, 1000);
}

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
}

function initCircuitAnimations() {
  const circuitPaths = document.querySelectorAll(".circuit-path");

  circuitPaths.forEach((path, index) => {
    path.style.animationDelay = `${index * 0.5}s`;
    path.style.strokeDasharray = `${Math.random() * 15 + 5}`;
  });
}

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
        header.style.background = "rgba(0, 255, 159, 0.1)";
      }
      card.style.transform = "";
      card.style.boxShadow = "";
    });
  });
}

function initTerminalEffects() {
  const terminals = document.querySelectorAll(".main-terminal, .terminal-window");

  terminals.forEach(terminal => {
    setInterval(() => {
      if (Math.random() > 0.98) {
        terminal.style.transform = `translateX(${Math.random() * 2}px)`;

        setTimeout(() => {
          terminal.style.transform = "none";
        }, 50);
      }
    }, 50);
  });

  const cursors = document.querySelectorAll(".cursor");
  cursors.forEach(cursor => {
    setInterval(() => {
      cursor.style.opacity = cursor.style.opacity === "0" ? "1" : "0";
    }, 500);
  });
}

function initTimestampUpdates() {
  function updateTimestamp() {
    const timestamp = document.getElementById("timestamp");
    if (timestamp) {
      const now = new Date();
      const formatted = now.toISOString().replace("T", " ").replace("Z", "");
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

document.addEventListener('htmx:beforeRequest', function (event) {
  const target = event.detail.target;
  if (target.classList.contains('project-grid')) {
    target.innerHTML = '<div class="loading">Initializing project matrix...</div>';
  }
});

document.addEventListener('htmx:beforeRequest', function (event) {
  const form = event.detail.elt;
  if (form.tagName === 'FORM') {
    const submitButton = form.querySelector('button[type="submit"]');
    if (submitButton) {
      submitButton.disabled = true;
      submitButton.innerHTML = 'transmitting...';
    }
  }
});

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
