# ThinkTerm - The Terminal That Thinks With You

![TT Logo](tt_logo.png)

ThinkTerm is a smart and customizable terminal emulator built in Rust, combining standard shell capabilities with AI-powered interactions. It allows users to execute shell commands, interact with AI, and maintain history with tab completion.

## Features

✅ **Execute Shell Commands** - Run standard Unix/Linux commands seamlessly.  
✅ **AI-Powered Assistant** - Ask AI questions with `ai <your question>` using OpenAI's API.  
✅ **Command History & Tab Completion** - Easily navigate through past commands.  
✅ **AI Memory** - Remembers previous interactions for context-aware responses.  
✅ **Custom Themes** - Modify prompt colors to match your style.  
✅ **Async Execution** - Uses `tokio` for non-blocking AI calls.  
✅ **Minimal Dependencies** - Optimized for performance with Rust's ecosystem.  

## Installation

### Prerequisites
- **Rust & Cargo** installed ([Download Rust](https://www.rust-lang.org/tools/install))
- **OpenAI API Key** (for AI interactions)

### Clone and Build
```sh
# Clone repository
git clone https://github.com/makalin/ThinkTerm.git
cd ThinkTerm

# Build and Run
cargo run
```

## Usage

### Running Shell Commands
ThinkTerm works like a regular shell. You can execute system commands such as:
```sh
ls -la
pwd
echo "Hello, ThinkTerm!"
```

### AI-Powered Assistant
You can use ThinkTerm’s built-in AI for assistance. Just type:
```sh
ai How do I write a loop in Rust?
```
ThinkTerm will fetch an AI-generated response from OpenAI.

### Command History & Tab Completion
- **Up/Down Arrow Keys**: Navigate command history.
- **Tab Completion**: Auto-completes commands for efficiency.

### Exiting ThinkTerm
- Type `exit` and press **Enter**.
- Use `Ctrl+D` to exit immediately.

## Configuration & Customization

### Changing Prompt Color
Modify the `theme_color` variable in `main.rs` to customize the terminal’s appearance. The default is **purple** (`\x1b[35m`). Example:
```rust
let theme_color = "\x1b[34m"; // Blue
```

### Increasing History Size
By default, ThinkTerm stores **100** previous commands. To change this, update:
```rust
const HISTORY_SIZE: usize = 200;
```

## Roadmap & Future Enhancements

🔹 **Persistent History Storage** – Save command history across sessions.  
🔹 **Custom AI Models** – Support for different AI backends (e.g., local models).  
🔹 **Plugin System** – Allow third-party extensions for added features.  
🔹 **Improved Performance** – Faster command execution and caching.  

## Contributing
We welcome contributions! To contribute:
1. **Fork** the repository.
2. **Create a feature branch** (`git checkout -b feature-branch`).
3. **Commit changes** (`git commit -m "Added new feature"`).
4. **Push to GitHub** (`git push origin feature-branch`).
5. **Open a pull request**.

For major changes, please open an issue first to discuss proposed changes.

## License
MIT License. See `LICENSE` file for details.

---
🚀 **ThinkTerm** - The future of terminals, now with AI!
