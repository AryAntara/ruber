# Ruber

**Ruber** is a lightweight, efficient web automation tool powered by a custom Domain-Specific Language (DSL). Built with Rust, it provides a simple and human-readable way to script browser interactions without the complexity of traditional automation frameworks.

It leverages [fantoccini](https://github.com/jonhoo/fantoccini) for WebDriver communication and [pest](https://pest.rs/) for syntax parsing.

---

## 🚀 Features

- **Human-Readable Syntax**: Write automation scripts using natural commands like `go to`, `click`, and `fill`.
- **Flow Control**: Group commands into reusable `flows` and even nest them for complex scenarios.
- **WebDriver Integration**: Works with any WebDriver-compatible browser (Chrome, Firefox, etc.).
- **Lightweight & Fast**: Compiled Rust binary with minimal overhead.

---

## 🛠️ Syntax Overview

Ruber uses a `.rub` script format. Below are the supported commands:

| Command | Description | Example |
| :--- | :--- | :--- |
| `go to <url>` | Navigates to the specified URL. | `go to https://google.com` |
| `click element <selector>` | Clicks on an element using a CSS selector. | `click element "#submit-button"` |
| `fill element <sel> with <val>` | Fills an input field with the specified text. | `fill element "#search" with "ruber rust"` |
| `wait for <num> <unit>` | Pauses execution. Units: `seconds`, `minutes`, `ms`. | `wait for 5 seconds` |
| `trigger <event> from <sel>` | Triggers a DOM event (e.g., `click`, `change`). | `trigger click from ".btn"` |
| `select first option for <sel>` | Selects the first option in a dropdown. | `select first option for "#dropdown"` |
| `press key <key>` | Simulates a key press (e.g., "Enter"). | `press key "Enter"` |
| `create flow <name> ... stop` | Defines a block of commands as a reusable flow. | See below |
| `using flow <name>` | Executes a previously defined flow. | `using flow login` |
| `include <file>` | Includes commands from another `.rub` file. | `include "auth.rub"` |
| `hang up` | Pauses the script and waits for user input (Ctrl+C). | `hang up` |

### 📝 Flow Example

```rust
create flow search_google
    go to https://google.com
    fill element "#APjFqb" with "Rust Programming"
    press key "Enter"
    wait for 2 seconds
stop

using flow search_google
```

---

## ⚙️ Configuration

Ruber looks for a `ruber.yaml` file in the execution directory (or path provided) to determine the entry point.

```yaml
index_file: main
```

This configuration tells Ruber to look for `main.rub` as the starting script.

---

## 🏁 Getting Started

### Prerequisites

1.  **Rust**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2.  **WebDriver**: You need a running WebDriver instance (e.g., `chromedriver` or `geckodriver`).
    ```bash
    # Example for Chrome
    chromedriver --port=4444
    ```

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/your-username/ruber.git
cd ruber
cargo build --release
```

### Running

1.  Create your `.rub` script (e.g., `main.rub`).
2.  Start your WebDriver.
3.  Run Ruber:
    ```bash
    cargo run
    ```

---

## 📁 Project Structure

- `src/main.rs`: Application entry point.
- `src/grammars/core.pest`: The PEG grammar definition for the Ruber DSL.
- `src/util/command.rs`: Command parser and execution logic.
- `src/util/client.rs`: WebDriver client initialization.
- `src/examples/`: Sample `.rub` scripts to help you get started.

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the DSL, add new commands, or enhance the documentation.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE) (or specify your license).
