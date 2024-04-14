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
        let a = caps.get(1).map_or(1.0, |m| m.as_str().parse::<f64>().unwrap_or(1.0));
        let b = caps.get(2).map_or(0.0, |m| m.as_str().replace(" ", "").parse::<f64>().unwrap_or(0.0));
        return Some((a, b));
    } else {
        None
    }
}

fn parse_and_evaluate(line: &str, functions: &mut Vec<(String, String)>) -> (Option<String>, Option<String>) {
    let function_def_regex = Regex::new(r"(\w+)\(x\):= (.+)").unwrap();
    let function_call_regex = Regex::new(r"(\w+)\((\d+)\)").unwrap();
    let function_solve_regex = Regex::new(r"(\w+) = (\d+)").unwrap();

    if let Some(caps) = function_def_regex.captures(line) {
        let func_name = caps.get(1).unwrap().as_str().to_string();
        let expression = caps.get(2).unwrap().as_str().to_string();
        functions.push((func_name.clone(), expression.clone()));
        return (Some(format!("Defined function {}: {}", func_name, expression).green().to_string()), None);
    }

    for (name, expr) in functions.iter() {
        if let Some(caps) = function_call_regex.captures(line) {
            if caps.get(1).unwrap().as_str() == *name {
                let x_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let result_expr = expr.replace("x", &x_value.to_string());
                let result = meval::eval_str(&result_expr).unwrap();
                return (Some(format!("{}({}) = {}", name, x_value, result).green().to_string()), None);
            }
        } else if let Some(caps) = function_solve_regex.captures(line) {
            if caps.get(1).unwrap().as_str() == *name {
                let target_value: f64 = caps.get(2).unwrap().as_str().parse().unwrap();
                if let Some((a, b)) = parse_linear_expr(expr) {
                    let x = (target_value - b) / a;
                    return (Some(format!("x = {:.2}", x).green().to_string()), None);
                }
            }
        }
    }

    return (None, Some("Function or proper form not found".red().to_string()));
}

fn main() -> Result<()> {
    let mut functions = Vec::new();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    let args: Vec<String> = env::args().collect();
    let default_file_name = "example.txt".to_string();
    let file_name = args.get(1).unwrap_or(&default_file_name);
    let path = Path::new(&file_name);

    let mut file = OpenOptions::new().read(true).write(true).create(true).open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<String> = contents.split('\n').map(String::from).collect();

    let mut current_line = 0;
    let mut cursor_position = 0;

    let (cols, rows) = size()?;
    let message_area_start = rows - 2;  // Reserve the last two lines for messages

    loop {
        execute!(stdout, terminal::Clear(ClearType::All))?;

        for (index, line) in lines.iter().enumerate() {
            if index as u16 >= message_area_start { break; }
            execute!(stdout, cursor::MoveTo(0, index as u16))?;
            writeln!(stdout, "{}", line)?;
        }

        execute!(stdout, cursor::MoveTo(0, message_area_start))?;

        execute!(stdout, cursor::MoveTo(cursor_position as u16, current_line as u16))?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(c) => {
                    lines[current_line].insert_str(cursor_position, &c.to_string());
                    cursor_position += 1;
                },
                KeyCode::Enter => {
                    if lines[current_line].trim_start().starts_with("//") {
                        lines.insert(current_line + 1, String::new());
                    } else {
                        let (result, error) = parse_and_evaluate(&lines[current_line], &mut functions);
                        if let Some(res) = result {
                            lines.insert(current_line + 1, res);
                            current_line += 1;
                        }
                        if let Some(err) = error {
                            // Display the error in the message area without altering the document
                            execute!(stdout, cursor::MoveTo(0, message_area_start))?;
                            writeln!(stdout, "{}", err)?;
                        }
                    }
                    lines.insert(current_line, String::new());
                    cursor_position = 0;
                },
                KeyCode::Backspace => {
                    if cursor_position > 0 {
                        lines[current_line].remove(cursor_position - 1);
                        cursor_position -= 1;
                    } else if current_line > 0 {
                        current_line -= 1;
                        cursor_position = lines[current_line].len();
                    }
                },
                KeyCode::Left => {
                    if cursor_position > 0 {
                        cursor_position -= 1;
                    }
                },
                KeyCode::Right => {
                    if cursor_position < lines[current_line].len() {
                        cursor_position += 1;
                    }
                },
                KeyCode::Up => {
                    if current_line > 0 {
                        current_line -= 1;
                        cursor_position = lines[current_line].len();
                    }
                },
                KeyCode::Down => {
                    if current_line < lines.len() - 1 {
                        current_line += 1;
                        cursor_position = 0;
                    }
                },
                KeyCode::Esc => {
                    let contents_to_save = lines.join("\n");
                    OpenOptions::new().write(true).truncate(true).open(&path)?.write_all(contents_to_save.as_bytes())?;
                    break;
                },
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}

