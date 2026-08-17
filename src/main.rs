mod formatter;
mod task;
mod token;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap().as_str() {
        "add" => {
            formatter::coder(&[task::create_task(&args)], "examples/tasks.md");
            ()
        }
        "list" => {
            match formatter::decode("examples/tasks.md") {
                Ok(tasks) => {
                    for task in tasks {
                        println!("{:#?}", task);
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            };
            ()
        }
        _ => (),
    }
}
