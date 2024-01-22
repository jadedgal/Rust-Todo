#[path = "add_task.rs"]
mod add_task;
#[path = "clear.rs"]
mod clear;
#[path = "get_tasks.rs"]
mod get_tasks;
use std::io;
fn remove_task(task: usize) {
    println!("Removing task number: {}", task + 1);
    let tasks = get_tasks::get_tasks();
    clear::clear();
    let mut x: usize = 0;
     loop {
        if x == tasks.len() {
          println!("Well done on completing the task: {}, it has been removed from the list",tasks[task]);
            break;
        }
        if x == task {
          x+=1;
            continue;
        }
        add_task::append(tasks[x].clone());
        x += 1;
    }
    
}
pub fn handle_remove() {
    get_tasks::output_tasks();
    println!("What task have you completed?");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");

    let task = task.trim();
    let task: usize = task.parse().expect("Please type a number!");

    println!("Are you sure? (Y/n)");
    let mut conf = String::new();
    io::stdin()
        .read_line(&mut conf)
        .expect("Failed to read line");
    let conf = conf.trim();
    if conf == "Y" || conf == "y" || conf == "" {
        remove_task(task - 1);
    } else {
        println!("Task not removed");
    }
}
