use crate::task;
use chrono::Local;

pub fn parse(args: &Vec<String>) -> task::Task {
    let current_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let mut task = task::Task {
        status: true,
        text: "".to_string(),
        tags: vec![],
        time: current_time,
        importance: 0,
    };

    for word in args.iter().skip(2) {
        match word.as_str() {
            //set task's importance equal count of '!'
            "!!!" | "!!" | "!" => task.importance = word.len(),
            _ if word.starts_with('@') => task.tags.push(word.to_string()),
            _ => {
                task.text.push_str(word);
                task.text.push(' ');
            }
        }
    }
    task
}
