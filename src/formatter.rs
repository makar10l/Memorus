use crate::task;
use std::io::BufRead;
use std::{fs::File, io};
//rewrite path to config reader
pub fn decode(path: &str) -> Result<Vec<task::Task>, String> {
    let mut tasks: Vec<task::Task> = vec![];
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            println!("{}", err.to_string());
            return Err(format!("Error in IO -> Can't open file '{}'", path).to_string());
        }
    };
    let buffer = io::BufReader::new(file);
    let mut temp_task = task::create_task();

    for (line_number, line) in buffer.lines().enumerate() {
        let mut _line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let mut tokens_iter = _line.as_str().split_whitespace();

        let key_word = tokens_iter.next().unwrap_or("");

        match key_word {
            "[]" | "[+]" => temp_task.status = key_word.contains("+"),

            "text" => {
                tokens_iter.next();
                temp_task.text = tokens_iter.next().unwrap_or("").to_string();
            }

            "text:" => {
                temp_task.text = tokens_iter.next().unwrap_or("").to_string();
            }

            "end." => {
                tasks.push(temp_task);
                temp_task = task::create_task();
            }

            _ => {
                return Err(format!(
                    "Error in line -> '{} : {}' : Unexpected symbol",
                    line_number + 1,
                    _line
                )
                .to_string());
            }
        }
    }
    Ok(tasks)
}
