use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write, Read, Result};
use std::path::Path;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    style::Stylize,
};
use regex::Regex;

// Helper function to parse expressions of the form `ax + b`
fn parse_linear_expr(expr: &str) -> Option<(f64, f64)> {
    let linear_regex = Regex::new(r"([+-]?\d*\.?\d*)x\s*([+-]?\s*\d+\.?\d*)").unwrap();
    if let Some(caps) = linear_regex.captures(expr) {
        let a = caps.get(1).map_or(1.0, |m| m.as_str().parse().unwrap_or(1.0));
        let b = caps.get(2).map_or(0.0, |m| m.as_str().replace(" ", "").parse().unwrap_or(0.0));
        Some((a, b))
    } else {
        None
    }
}

// Helper to parse and evaluate expressions or definitions
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
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    // Retrieve the command line arguments and determine the file path
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap_or(&"example.txt".to_string()).clone();
    let path = Path::new(&file_name);

    // Open or create the file at the specified path
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let mut contents = String::new();
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
                    if let Some(result) = parse_and_evaluate(&lines[current_line], &mut functions) {
                        lines.insert(current_line + 1, result);
                    } else {
                        lines.insert(current_line + 1, "Syntax Error".red().to_string());
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
