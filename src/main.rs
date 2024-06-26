use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write, Read, Result};
use std::path::Path;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Stylize,
    terminal::{self, ClearType, size},
};
use regex::Regex;

fn parse_linear_expr(expr: &str) -> Option<(f64, f64)> {
    let linear_regex = Regex::new(r"([+-]?\d*\.?\d*)x\s*([+-]?\s*\d+\.?\d*)").unwrap();
    if let Some(caps) = linear_regex.captures(expr) {
        let a = caps.get(1).map_or(1.0, |m| m.as_str().parse().unwrap_or(1.0));
        let b = caps.get(2).map_or(0.0, |m| m.as_str().replace(" ", "").parse().unwrap_or(0.0));
        return Some((a, b));
    } else {
        None
    }
}

fn parse_and_evaluate(line: &str, functions: &mut Vec<(String, String)>) -> Option<String> {
    let function_def_regex = Regex::new(r"(\w+)\(x\):= (.+)").unwrap();
    let function_call_regex = Regex::new(r"(\w+)\((\d+)\)").unwrap();
    let function_solve_regex = Regex::new(r"(\w+) = (\d+)").unwrap();

    if let Some(caps) = function_def_regex.captures(line) {
        let func_name = caps.get(1).unwrap().as_str().to_string();
        let expression = caps.get(2).unwrap().as_str().to_string();
        functions.push((func_name.clone(), expression));
        return Some(format!("Defined function {}", func_name).green().to_string());
    }

    if let Some(caps) = function_call_regex.captures(line) {
        let func_name = caps.get(1).unwrap().as_str();
        let x_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        for (name, expr) in functions.iter() {
            if name == func_name {
                let result_expr = expr.replace("x", &x_value.to_string());
                let result = meval::eval_str(&result_expr).unwrap();
                return Some(format!("{}({}) = {}", func_name, x_value, result).green().to_string());
            }
        }
        return Some("Function not found".red().to_string());
    }

    if let Some(caps) = function_solve_regex.captures(line) {
        let func_name = caps.get(1).unwrap().as_str();
        let target_value: f64 = caps.get(2).unwrap().as_str().parse().unwrap();
        for (name, expr) in functions.iter() {
            if name == func_name {
                if let Some((a, b)) = parse_linear_expr(expr) {
                    let x = (target_value - b) / a;
                    return Some(format!("x = {:.2}", x).green().to_string());
                }
            }
        }
        return Some("Function or proper form not found".red().to_string());
    }

    None
}

fn main() -> Result<()> {
    let mut functions = Vec::new();
    let mut logs = Vec::new();  // Initialize the logs vector
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    let args: Vec<String> = env::args().collect();
    let default_file_name = "example.txt".to_string();
    let file_name = args.get(1).unwrap_or(&default_file_name);
    let path = Path::new(file_name);

    let mut file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

    let mut current_line = 0;
    let mut cursor_position = 0;

    let (cols, rows) = size()?;
    let message_area_start = rows / 3 * 2;  // Allocate the lower third for messages

    loop {
        execute!(stdout, terminal::Clear(ClearType::All))?;

        for (index, line) in lines.iter().enumerate() {
            if index as u16 >= message_area_start { break; }
            execute!(stdout, cursor::MoveTo(0, index as u16))?;
            writeln!(stdout, "{}", line)?;
        }

        execute!(stdout, cursor::MoveTo(0, message_area_start))?;
        for log in &logs {
            writeln!(stdout, "{}", log)?;
        }
        logs.clear();

        execute!(stdout, cursor::MoveTo(cursor_position as u16, current_line as u16))?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(c) => {
                    lines[current_line].insert_str(cursor_position as usize, &c.to_string());
                    cursor_position += 1;
                },
                KeyCode::Enter => {
                    if lines[current_line].trim_start().starts_with("//") {
                        lines.insert(current_line + 1, String::new()); // Just insert a new line for comments
                    } else if let Some(result) = parse_and_evaluate(&lines[current_line], &mut functions) {
                        logs.clear(); // Clear previous log
                        logs.push(result); // Show only the latest result
                    } else {
                        logs.clear(); // Clear previous log
                        logs.push("Syntax Error".red().to_string());
                    }
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
                        cursor_position = lines[current_line].len() as u16;  // Convert usize to u16
                    }
                },
                KeyCode::Left => {
                    if cursor_position > 0 {
                        cursor_position -= 1;
                    }
                },
                KeyCode::Right => {
                    if cursor_position < lines[current_line].len() as u16 {
                        cursor_position += 1;
                    }
                },
                KeyCode::Up => {
                    if current_line > 0 {
                        current_line -= 1;
                        cursor_position = lines[current_line].len().min(cursor_position as usize) as u16;
                    }
                },
                KeyCode::Down => {
                    if current_line < lines.len() - 1 {
                        current_line += 1;
                        cursor_position = 0;
                    }
                },
                KeyCode::Esc => {
                    let contents_to_save: Vec<String> = lines
                        .iter()
                        .filter(|line| !line.starts_with("//"))
                        .cloned()
                        .collect();
                    let contents = contents_to_save.join("\n");
                    OpenOptions::new().write(true).truncate(true).open(path)?.write_all(contents.as_bytes())?;
                    break;
                },
                _ => {} // Handle other KeyCode cases if necessary
            },
            Event::Mouse(_) => {}, // Handle mouse events if necessary
            Event::Resize(_, _) => {}, // Handle terminal resize events
            Event::FocusGained | Event::FocusLost => {}, // Handle focus gain/loss if necessary
            Event::Paste(_) => {}, // Handle paste events if necessary
            _ => {} // This covers any other unhandled events
        }
        
    }
    Ok(())
}
