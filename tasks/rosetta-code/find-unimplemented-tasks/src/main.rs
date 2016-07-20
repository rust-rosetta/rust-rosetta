extern crate find_unimplemented_tasks;

fn main() {
    for task in find_unimplemented_tasks::unimplemented_tasks("Rust") {
        println!("{:6} {}", task.id, task.title);
    }
}
