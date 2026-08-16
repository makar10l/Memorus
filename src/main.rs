mod formatter;
mod task;
mod token;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = task::create_task(&args);
    // println!("{:?}", task);
    // match formatter::decode("tests/tasks.md") {
    //     Ok(tasks) => {
    //         for task in tasks {
    //             println!("{:#?}", task);
    //         }
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // }
    formatter::coder(&[task], "tests/tasks.md");
}
