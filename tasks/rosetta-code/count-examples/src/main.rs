
extern crate count_examples;
fn main() {
    let all_tasks = count_examples::query_all_tasks();
    for task in &all_tasks {
        let count = count_examples::query_a_task(task);
        println!("Task: {} has {} examples", task.title, count);
    }
}
