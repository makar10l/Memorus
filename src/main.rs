mod task;
mod markdown;
fn main(){
    markdown::deserialize("tasks.md");
}