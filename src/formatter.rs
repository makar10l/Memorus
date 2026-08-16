use crate::task;
use crate::token;
use std::fs::OpenOptions;
use std::io::{BufRead, BufWriter, Write};
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
    let mut temp_task = task::create_empty_task();

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

            "timestamp" => {
                tokens_iter.next();
                temp_task.timestamp = tokens_iter.next().unwrap_or("").to_string();
            }

            "importance" => {
                tokens_iter.next();
                temp_task.importance = tokens_iter.next().unwrap_or("").parse().unwrap_or(0);
            }

            "tags" => {
                tokens_iter.next();
                for tag in tokens_iter {
                    temp_task.tags.push(tag.to_string())
                }
            }
            "end." => {
                tasks.push(temp_task);
                temp_task = task::create_empty_task();
            }
            "" => continue,
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

pub fn coder(tasks: &[task::Task], path: &str) -> Result<(), token::Error<std::io::Error>> {
    let file = match OpenOptions::new().append(true).open(path) {
        Ok(fd) => fd,
        Err(err) => return Err(token::Error::UndefinedError(err)),
    };
    let mut writer = BufWriter::new(file);

    for task in tasks {
        token::status(&mut writer, &task)?;
        token::text(&mut writer, &task)?;
        token::importance(&mut writer, &task)?;
        token::tags(&mut writer, &task)?;
        token::timestamp(&mut writer, &task)?;
        match writer.write_all("end.\n".as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(token::Error::UndefinedError(err)),
        };
        match writer.flush() {
            Ok(_) => (),
            Err(err) => return Err(token::Error::UndefinedError(err)),
        };
    }
    Ok(())
}
