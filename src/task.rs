use chrono::Local;

#[derive(Debug)]
pub struct Task {
    pub status: bool,
    pub text: String,
    pub tags: Vec<String>,
    pub timestamp: String,
    pub importance: usize,
}

impl Task {
    pub fn add(&self) {}
    pub fn list() {}
}

#[inline(always)]
pub fn create_empty_task() -> Task {
    Task {
        status: false,
        text: "".to_string(),
        tags: vec![],
        timestamp: "".to_string(),
        importance: 0,
    }
}

pub fn create_task(args: &[String]) -> Task {
    let current_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let mut task = create_empty_task();
    task.timestamp = current_time;
    task.status = true;

    for word in args.iter().skip(2) {
        match word.as_str() {
            //set task's importance equal count of '^'
            _ if word.starts_with("^") => task.importance = word.len(),
            _ if word.starts_with('@') => task.tags.push(word.to_string()),
            _ => {
                task.text.push_str(word);
                task.text.push(' ');
            }
        }
    }
    task
}
