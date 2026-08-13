mod parser;
mod task;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = parser::parse(&args).unwrap();
    println!("{:?}", task);
}
