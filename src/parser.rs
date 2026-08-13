use crate::task;
use chrono::Local;
pub fn parse(args: &Vec<String>) -> Option<task::Task> {
    let current_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let mut task = task::Task {
        status: true,
        text: "".to_string(),
        tags: vec![],
        time: current_time,
        importance: 0,
    };

    for word in args.iter().skip(1) {
        match word.as_str() {
            "add" => (),
            "list" => {
                task.status = false;
                task::Task::list()
            }
            "!!!" | "!!" | "!" => task.importance = word.len(),
            _ if word.starts_with('@') => task.tags.push(word.to_string()),
            _ => {
                task.text.push(' ');
                task.text.push_str(word);
            }
        }
    }
    if task.status {
        return Some(task);
    }
    None
}
