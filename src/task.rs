#[derive(Debug)]
pub struct Task {
    pub status: bool,
    pub text: String,
    pub tags: Vec<String>,
    pub time: String,
    pub importance: usize,
}

impl Task {
    pub fn add(&self, path: &str) {}
    pub fn list() {}
}
