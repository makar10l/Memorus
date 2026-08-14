#[derive(Debug)]
pub struct Task {
    pub status: bool,
    pub text: String,
    pub tags: Vec<String>,
    pub time: String,
    pub importance: usize,
}

impl Task {
    pub fn add(&self) {}
    pub fn list() {}
}
#[inline(always)]
pub fn create_task() -> Task {
    Task {
        status: false,
        text: "".to_string(),
        tags: vec![],
        time: "".to_string(),
        importance: 0,
    }
}
