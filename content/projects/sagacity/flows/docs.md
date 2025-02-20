## Sagacity

Sagacity is a command-line application written in Rust that aims to help developers explore and understand their codebase more effectively. It leverages natural language processing and AI models from Anthropic to provide relevant information about the codebase in response to natural language queries.

### Key Features

1. **Codebase Indexing**:
   - Sagacity scans the specified codebase directory and indexes all the source files (Rust, Python, Go, JavaScript, TypeScript, Java, C, C++, Markdown, and Toml files).
   - It generates concise summaries for each file using Anthropic's Claude AI model, making it easier to understand the purpose and functionality of different components.

2. **Natural Language Queries**:
   - Users can ask questions about the codebase in natural language, and Sagacity will provide relevant information based on the indexed files and summaries.
   - The application supports a conversational mode, allowing users to have an interactive dialogue with the AI assistant.

3. **Contextual Responses**:
   - Sagacity takes into account the conversation history and relevant code context to provide accurate and contextual responses.
   - It intelligently searches and ranks the indexed files based on their relevance to the user's query, ensuring that the most pertinent information is presented.

4. **Interactive CLI**:
   - Sagacity offers an interactive command-line interface (CLI) with various features for seamless interaction and navigation.
   - Users can browse and view summaries for individual files, manage conversations, and perform various actions like copying responses to the clipboard or saving them to files.

5. **Response Management**:
   - Responses from the AI can be easily copied to the clipboard or saved to files for future reference.
   - Sagacity maintains a conversation history, allowing users to review previous queries and responses.

6. **Caching and Optimization**:
   - Sagacity implements caching mechanisms to avoid redundant indexing and summaries, improving performance and reducing API call costs.
   - It tracks file modification times and only re-indexes files that have been modified since the last run.

7. **Token and Cost Tracking**:
   - Sagacity keeps track of the number of tokens used for input, cache writes, cache hits, and output, providing transparency into the token usage and associated costs.
   - It calculates and displays the total token usage and estimated cost for each interaction, helping users manage their API usage and budget.

8. **Extensibility**:
   - The project is designed with a modular structure and separation of concerns, making it easier to maintain and extend the codebase in the future.
   - The integration with Anthropic's Claude AI model can be extended or replaced with other language models if desired.

### Installation and Setup

Sagacity can be installed using Cargo, the Rust package manager. It requires an API key from Anthropic, which needs to be added to the user's `.zshrc` file (or the appropriate shell configuration file) as an environment variable.

Please refer to the project's README file for detailed instructions on installation, setup, and usage.

### Usage

After installation, users can run the `sagacity` command from the terminal to start the interactive CLI. The main menu provides options to enter chat mode, browse the indexed files, view help, and quit the application.

In chat mode, users can ask questions about the codebase in natural language, and Sagacity will provide relevant information based on the indexed files and summaries. Various commands are available to manage the conversation, such as clearing the history, saving/loading conversations, and exiting the chat mode.

### Codebase Structure

The Sagacity codebase is organized into several modules and files:

- `main.rs`: The entry point of the application, containing the main function and high-level logic.
- `constants.rs`: Defines constants used throughout the application, such as API URLs, models, and UI constants.
- Structs and enums: Sagacity defines several structs and enums to represent data structures like messages, conversations, API call logs, and index caches.
- Helper functions: The codebase includes various helper functions for tasks like file operations, API calls, codebase indexing, response generation, and more.

The project follows best practices for Rust development, including error handling, asynchronous programming with Tokio, and modular design.

### Dependencies

Sagacity relies on several external crates and libraries, including:

- `reqwest`: For making HTTP requests to the Anthropic API.
- `serde`: For serialization and deserialization of data structures.
- `ignore`: For handling Git ignore patterns during codebase scanning.
- `dialoguer`: For creating interactive command-line interfaces.
- `rustyline`: For providing a readline-like interface for user input.
- `indicatif`: For displaying progress bars and spinners.
- `prettytable`: For displaying tabular data in the terminal.
- `chrono`: For handling date and time operations.
- `claude-tokenizer`: For tokenizing and counting tokens in prompts and responses.

Please refer to the `Cargo.toml` file for the complete list of dependencies and their versions.
