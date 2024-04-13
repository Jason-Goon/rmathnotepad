use std::env;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    Result,
};
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::path::Path;


fn main() -> Result<()> {
    let mut _stdout = io::stdout();
    terminal::enable_raw_mode()?;

    let _args: Vec<String> = env::args().collect();
    let path = Path::new("./example.txt");
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(&path)?;
    let mut stdout = io::stdout();

    let mut contents = String::new();
    file.read_to_string(&mut contents)?; 
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

    let mut current_line = 0;
    let mut cursor_position = 0;

    loop {
        execute!(stdout, terminal::Clear(ClearType::All))?;
        for (index, line) in lines.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, index as u16))?;
            writeln!(stdout, "{}", line)?;
        }

        execute!(stdout, cursor::MoveTo(cursor_position, current_line as u16))?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(c) => {
                    lines[current_line].insert_str(cursor_position as usize, &c.to_string());
                    cursor_position += 1;
                },
                KeyCode::Enter => {
                    current_line += 1;
                    lines.insert(current_line, String::new());
                    cursor_position = 0; 
                },
                KeyCode::Backspace => {
                    if cursor_position > 0 {
                        lines[current_line].remove((cursor_position - 1) as usize);
                        cursor_position -= 1;
                    } else if current_line > 0 {
                        current_line -= 1;
                        cursor_position = lines[current_line].len() as u16;
                        let next_line = lines.remove(current_line + 1);
                        lines[current_line].push_str(&next_line);
                    }
                },
                KeyCode::Left => {
                    if cursor_position > 0 {
                        cursor_position -= 1;
                    };
                },
                KeyCode::Right => {
                    if cursor_position < lines[current_line].len() as u16 {
                        cursor_position += 1;
                    }
                },
                KeyCode::Up => {
                    if current_line > 0 {
                        current_line -= 1;
                        cursor_position = lines[current_line].len() as u16;
                    }
                },
                KeyCode::Down => {
                    if current_line < lines.len() - 1 {
                        current_line += 1;
                        cursor_position = 0;
                    }
                },
                KeyCode::Esc => {
                    let contents = lines.join("\n");
                    OpenOptions::new().write(true).truncate(true).open(path)?.write_all(contents.as_bytes())?;
                    break;
                },
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
