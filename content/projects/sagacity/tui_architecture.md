# TUI Architecture Documentation

This document explains the architecture of the terminal user interface (TUI) used throughout the application. It covers the overall directory structure, key screens, layout components, key handlers, and how user input is processed to deliver an interactive, production-grade experience.

---

## Table of Contents

1. [Directory Structure](#directory-structure)
2. [Screens and Layouts](#screens-and-layouts)
   - [Splash Screen](#splash-screen)
   - [Indexing Screen](#indexing-screen)
   - [Chat Screen](#chat-screen)
   - [Database Details Screen](#database-details-screen)
3. [Key Handlers and User Input](#key-handlers-and-user-input)
4. [UI Components and Rendering](#ui-components-and-rendering)
5. [State Management and Transitions](#state-management-and-transitions)
6. [Conclusion](#conclusion)

---

## Directory Structure

The source code is organized into several modules that each handle a specific aspect of the TUI:

- **src/**
  - **build.rs** – Build script that sets up environment variables.
  - **chat_view.rs** – Renders the main chat interface including message panels, input areas, and logs.
  - **chat_message.rs** – Defines the structure and rendering of individual chat messages (split into chunks such as text, code, or steps).
  - **code_snippet.rs** – Manages code snippets, including language detection and snippet selection.
  - **db_details_view.rs** – Renders the database details screen.
  - **indexing_view.rs** – Provides the UI for the codebase indexing process, including progress and file tree panels.
  - **log_view.rs** – Implements the logging panel that displays runtime logs.
  - **splash_screen.rs** – Renders the initial splash screen with a menu.
  - **status_indicator.rs** – Displays real-time status messages (e.g., processing spinners).
  - **db.rs** – Handles database connectivity and migrations.
  - Additional modules (e.g., **chat_message.rs**, **code_snippet.rs**) contribute to the overall functionality.

- **migrations/** – Contains SQL migration scripts that manage the database schema.

---

## Screens and Layouts

The TUI is divided into multiple screens, each with its own layout and purpose. The primary screens include:

### Splash Screen

- **Purpose:**
  Acts as the entry point to the application.

- **Features:**
  - Displays ASCII art and a menu with options such as "Start Chat", "DB Details", and "Quit".
  - Allows navigation using arrow keys and selection with Enter.

- **Module:**
  Implemented in `splash_screen.rs`.

### Indexing Screen

- **Purpose:**
  Provides visual feedback during the codebase indexing process.

- **Layout:**
  - **Status Header:** Displays a spinner, the number of files indexed, and elapsed time.
  - **File Tree Panel:** Lists files to be indexed with a progress bar and color indicators.
  - **Overall Progress Panel:** Shows a combined progress bar for all files.
  - **Logs Panel:** Displays real-time logs on the right side.

- **Module:**
  Implemented in `indexing_view.rs`.

### Chat Screen

- **Purpose:**
  The main interface for interacting with the AI assistant.

- **Layout:**
  - **Message Panel:** Shows the conversation history. Each message is broken down into chunks (text, code blocks, instructions).
  - **Input Field:** Where the user types messages.
  - **Status Indicator:** Displays system status such as "Thinking..." with animated spinners.
  - **Logs Panel:** Provides real-time logging of events.

- **Modules:**
  Implemented in `chat_view.rs` and supported by `chat_message.rs`.

### Database Details Screen

- **Purpose:**
  Displays detailed information about the database schema, migrations, and connection details.

- **Layout:**
  - **Details Grid:** Shows connection details (URL, version) and metadata (table count, migrations).
  - **Tables Grid:** Lists table names and their SQL definitions.
  - **Markdown Instructions:** Provides SQLx CLI instructions for managing the database.

- **Module:**
  Implemented in `db_details_view.rs`.

---

## Key Handlers and User Input

The TUI implements a robust key handling mechanism to support navigation and interaction across screens:

- **Global Key Events:**
  - **Esc:** Cancels indexing or exits a focused message.
  - **Ctrl+C:** Exits the application.

- **Screen-Specific Handlers:**
  - **Splash Screen:**
    - Up/Down arrows for menu navigation.
    - Enter key to select an option.

  - **Indexing Screen:**
    - Logs “Esc” to cancel indexing and transition to the Chat Screen.

  - **Chat Screen:**
    - **Arrow Keys:** Navigate through chat messages and focus individual message chunks.
    - **Enter:** Sends a message or copies the currently focused chunk if one is selected.
    - **Backspace, PageUp, PageDown:** Manage text input and scrolling.
    - **Colon (:)**: Enters command mode for advanced actions (e.g., copying a code block using `:copy 1` or focusing a message using `:focus 2`).

- **Command Mode:**
  In this mode, users can execute commands such as:
  - `:copy <n>` – Copies the nth code block from the focused message.
  - `:focus <n>` – Sets focus to a specific message in the conversation.
  - `:help` – Displays available commands.

- **Implementation:**
  Key handling is implemented across modules such as `chat_view.rs`, `splash_screen.rs`, and in the main event loop in `main.rs`.

---

## UI Components and Rendering

The TUI leverages the **ratatui** library for drawing components:

- **Panels and Layouts:**
  Flexible layout managers (using horizontal and vertical splits) allow screens to be composed of multiple widgets.

- **Widgets:**
  - **Paragraphs:** Used for displaying text (e.g., logs, messages, instructions).
  - **Blocks:** Provide borders and titles for visual grouping.

- **Custom Rendering:**
  - **Message Rendering:** Each chat message is rendered by iterating through its chunks, applying different styles based on whether a chunk is focused.
  - **Progress Bars and Spinners:** Dynamically rendered using formatted strings and animated frames.

---

## State Management and Transitions

- **Application State:**
  The main application state is maintained in the `App` struct (defined in `main.rs`), which holds the current screen, chat messages, indexing status, and other UI state variables.

- **Screen Transitions:**
  Transitions between screens (Splash, Indexing, Chat, DB Details) are controlled by state changes triggered by key events or completion of background tasks.

- **Concurrency:**
  The system uses asynchronous locks (via `tokio::sync::Mutex`) to safely update shared state across concurrent tasks, such as background indexing and chat response simulation.

---

## Conclusion

The TUI architecture is designed to provide a seamless and interactive user experience in the terminal. By organizing the code into dedicated modules, defining clear screen layouts, and implementing comprehensive key handlers, the system enables users to navigate complex information—such as codebase indexing, dynamic chat conversations, and detailed database views—with ease. The innovative approach to message chunking and code block selection further transforms the terminal interface into a powerful development workspace.
