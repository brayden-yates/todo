# Todo

A simple command-line to-do list application written in Rust. This application allows you to add tasks, mark them as done, list all pending tasks, and wipe the entire to-do list.

## Features

- **Add Tasks**: Easily add new tasks to your to-do list.
- **Mark Tasks as Done**: Update the status of your tasks to completed.
- **List Tasks**: View all tasks that are still pending.

## Requirements

- [Rust](https://www.rust-lang.org/) (with `cargo`) installed on your system.

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/todo_app.git
   cd todo_app
   ```

2. **Add dependencies**: Open `Cargo.toml` and make sure you have the following dependencies:

   ```toml
   [dependencies]
   rusqlite = "0.30"  # or your current version
   home = "0.5"
   ```

3. **Build the application**:

   ```bash
   cargo build --release
   ```

4. **Install the binary**: You can move the binary to a directory in your `PATH`, like `~/.local/bin`:

   ```bash
   mkdir -p ~/.local/bin
   cp target/release/todo_app ~/.local/bin/todo
   ```

   Make sure `~/.local/bin` is in your `PATH`. You can check your `PATH` by running:

   ```bash
   echo $PATH
   ```

   If it's not there, you can add it to your shell configuration file (e.g., `~/.bashrc` or `~/.zshrc`):

   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

   Then, run:

   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

## Usage

Once installed, you can use the following commands:

- **Add a Task**:
  ```bash
  todo add "Your task description here"
  ```

- **Mark a Task as Done**:
  ```bash
  todo done <task_id>
  ```

- **List All Tasks**:
  ```bash
  todo list
  ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue to discuss changes or improvements.
