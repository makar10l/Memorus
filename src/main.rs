mod formatter;
mod parser;
mod task;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = parser::parse(&args);
    println!("{:?}", task);
    let tasks = formatter::decode("tests/tasks.md").unwrap();
    for task in tasks {
        println!("{:?}", task);
    }
}
