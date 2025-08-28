# Retro Chat built in Rust

## Why I Built This
Honestly, I made this project partly to again pick up my interest in Rust and partly because I was bored.  


## Installing Rust & Cargo

This project requires Rust and Cargo. Cargo is Rust’s build tool and package manager, and it comes bundled with Rust.

### Linux & macOS
1. Install Rustup (includes Cargo):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. Follow the prompts (press 1 for default install).

3. Add Cargo to your PATH if it’s not already:
   ```
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
4. Verify the install
   ```
   cargo --version
   rustc --version
## Running the chat
1. Start the server file
   ```
   cargo run --bin server
2. Start the client file
   ```
   cargo run --bin client <username>
   
### Controls
- Enter → send message
- q or Esc → quit
- Messages window → auto-scrolls to new messages
- System notifications → highlighted when users join/leave

### Implementation Details
- Tokio → async networking & concurrency
- Cursive → retro TUI with ncurses backend
- Serde + JSON → lightweight message serialization
- Broadcast channel → fan-out to all connected clients
  
### Roadmap
- End-to-end encryption support
- Chat history persistence
- WebSocket bridge for browser clients
- Retro sound effects (because why not?)
