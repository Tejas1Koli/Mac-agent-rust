# mac-agent-rust 

A terminal-based macOS automation agent written in Rust. It uses [Ollama](https://ollama.com/) and the [Rig](https://github.com/0xPlaygrounds/rig) library to translate natural language commands into AppleScript, allowing you to control your Mac using conversational AI.

## Features

* **Natural Language Control:** Ask your Mac to perform tasks like "display a notification", "Spotify next track", or "create a reminder".
* **Local AI Execution:** Powered by Ollama for fast, private, and local execution.
* **AppleScript Integration:** Seamlessly interacts with macOS native applications and system settings.
* **Rust & Tokio:** Built for performance and reliability using async Rust.

## Prerequisites

* **macOS:** Required since the agent uses AppleScript (`osascript`) to execute commands.
* **Rust:** Set up via [rustup](https://rustup.rs/).
* **Ollama:** Install from [ollama.com](https://ollama.com/) and ensure it is running in the background.

## Dependencies

* `rig-core` and `rig` for the LLM agent framework.
* `tokio` for async runtime execution.
* `serde` and `serde_json` for serialization.
* `anyhow` and `thiserror` for error handling.


## Direct Usage(for cli interface)
```bash
cargo install mac-agent-rust
mac-agent-rust run
```


## Setup(for dev)

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Tejas1Koli/mac-agent-rust.git
   cd mac-agent-rust
   ```

2. **Pull the LLM text model:**
   The default model is `gemma4:e4b-mlx"` 
   ```bash
   ollama pull gemma4:e4b-mlx
   ```

## Configuration

Configuration cant be changed for now (for only cli interface)

* `OLLAMA_HOST`: URL to your Ollama instance (default: `http://localhost:11434`)
* `MODEL`: The LLM model to use
* `TEMPERATURE`: The creativity/randomness of the model (default: `0.7`)






### Special Commands
- `quit` or `exit`: Stop the agent.
- `reset`: Clear the chat history context.

