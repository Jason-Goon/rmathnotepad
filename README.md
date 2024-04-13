# Terminal Notepad & Future Programmable Calculator

This project is currently a terminal-based notepad application designed to run in Unix-like environments. It serves as the foundational step towards developing a minimal programmable calculator. The notepad application allows for basic text editing directly in the terminal, with future plans to incorporate programmable calculator features.

## Features

- **Basic Text Editing**: Create, edit, and view text files directly in the terminal.
- **Navigation**: Move the cursor using arrow keys, and perform edits with backspace, delete, and enter keys.
- **Persistence**: Save edits to disk, allowing for continued editing in future sessions.

## Planned Features

- **Programmable Calculator**: Extend the notepad to include calculator functionalities where users can perform arithmetic operations and store results within the text files.
- **Enhanced Navigation**: Improve text navigation and editing capabilities.

## Installation

To set up this project, you need Rust and Cargo installed on your machine.

1. **Clone the Repository:**
   ```bash
   git clone https://your-repository-url.git
   cd your-repository-directory
   ```

2. **Build the Project:**
   ```bash
   cargo build --release
   ```

3. **Run the Application:**
   ```bash
   cargo run --release
   ```

## Usage

Once you run the application, you are presented with a simple text interface in the terminal. Here are the key commands:

- **Arrow Keys**: Move the cursor within the text.
- **Enter**: Insert a new line.
- **Backspace**: Delete the character behind the cursor.
- **Esc**: Save and exit the application.

## Contributing

Contributions to extend both the notepad and the calculator functionalities are welcome. Before making a contribution, please fork the repository and submit a pull request for review.

## License

This project is licensed under the Zero-Clause BSD License - see the LICENSE file for details.
```
