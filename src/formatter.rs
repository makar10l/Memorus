use crate::task;
use std::io::BufRead;
use std::{fs::File, io};
//rewrite path to config reader`
pub fn decode(path: &str) -> Option<Vec<task::Task>> {
    let mut tasks: Vec<task::Task> = vec![];
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            println!("{}", err.to_string());
            return None;
        }
    };
    let buffer = io::BufReader::new(file);
    let mut temp_task = task::Task {
        status: false,
        text: "".to_string(),
        tags: vec![],
        time: "".to_string(),
        importance: 0,
    };

    for line in buffer.lines() {
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
                temp_task = task::Task {
                    status: false,
                    text: "".to_string(),
                    tags: vec![],
                    time: "".to_string(),
                    importance: 0,
                };
            }
            _ => return None,
        }
    }
    Some(tasks)
}
// jfljdlfjdf
// df;jfsldjfl
// [] 2026.29.02 12.12
// text :
// tags : @deadbeff
// importance :
