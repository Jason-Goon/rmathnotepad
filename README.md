```markdown
# Programmable Calculator

## Overview
This project provides a terminal-based notepad-style calculator capable of parsing and evaluating mathematical expressions and user-defined functions. It leverages the Rust programming language and the Crossterm library for terminal handling.

## Features
- **Expression Evaluation:** Parse and evaluate linear expressions and other calculations directly from the terminal.
- **Function Definition and Usage:** Users can define functions using a simple syntax and use these functions for various calculations.
- **Interactive Terminal Interface:** Utilizes raw mode for terminal input handling, allowing for interactive editing of expressions and functions.

## Requirements
- Rust Programming Language
- Crossterm Library
- Regex Library
- Meval Crate for evaluating mathematical expressions

## Usage
1. **Start the Program:** Run the program using `cargo run`. Optionally, specify a file name to load previous expressions and functions.
2. **Define Functions:** Use the syntax `functionName(x):= expression` to define functions. eq. "f(x):= 999x + 99" Whitespace/s required.
3. **Evaluate Expressions:** Simply type expressions to evaluate them. Use defined functions in expressions, automatically solves in line change if syntax is correct e.q. "f = 9" will return "x = -0.09"
4. **Control Keys:**
   - **Enter:** Evaluate the current line or insert a new line.
   - **Backspace:** Delete the character behind the cursor.
   - **Arrow Keys:** Navigate through the text.
   - **Esc:** Save the current session (excluding comments) and exit the program.

## Installation
To set up the programmable calculator, follow these steps:
```bash
git clone https://github.com/Jason-Goon/rmathnotepad
cd rmathnotepad
cargo build --release
cargo run --release i_am_a_file.txt
```
