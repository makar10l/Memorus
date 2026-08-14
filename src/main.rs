mod formatter;
mod parser;
mod task;
fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let task = parser::parse(&args);
    // println!("{:?}", task);
    match formatter::decode("tests/tasks.md") {
        Ok(tasks) => {
            for task in tasks {
                println!("{:?}", task);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
