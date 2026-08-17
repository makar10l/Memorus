use crate::task;
use std::fs::File;
use std::io::{BufWriter, Write};

pub enum Error<T> {
    TokenWriteError,
    UndefinedError(T),
}

pub fn text<E>(writer: &mut BufWriter<File>, task: &task::Task) -> Result<(), Error<E>> {
    match writer.write_all(format!("text : {}\n", task.text).as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::TokenWriteError),
    };
}

pub fn importance<E>(writer: &mut BufWriter<File>, task: &task::Task) -> Result<(), Error<E>> {
    match writer.write_all(format!("importance : {}\n", task.importance).as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::TokenWriteError),
    }
}

pub fn status<E>(writer: &mut BufWriter<File>, task: &task::Task) -> Result<(), Error<E>> {
    match writer.write_all({ if task.status { "[+]\n" } else { "[]\n" } }.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::TokenWriteError),
    }
}

pub fn timestamp<E>(writer: &mut BufWriter<File>, task: &task::Task) -> Result<(), Error<E>> {
    match writer.write_all(format!("timestamp : {}\n", task.timestamp).as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::TokenWriteError),
    }
}

pub fn tags<E>(writer: &mut BufWriter<File>, task: &task::Task) -> Result<(), Error<E>> {
    let mut tags = "tags : ".to_string();
    for tag in task.tags.iter() {
        tags.push_str(tag.as_str());
        tags.push(' ');
    }
    tags.trim();
    tags.push('\n');

    match writer.write_all(tags.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(Error::TokenWriteError),
    }
}
