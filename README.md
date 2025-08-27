# Retro Chat built in Rust





## ⚙️ Installing Rust & Cargo

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
