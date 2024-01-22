use std::fs::File;
use std::io::Read;

pub fn get_tasks() -> Vec<String> {
    let mut file = File::open("tasks.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let task_vect: Vec<&str> = contents.split('\n').collect();
    let mut output_task_vect = Vec::new();
    for task in task_vect {
        output_task_vect.push(task.to_string())
    }
    output_task_vect
}

pub fn output_tasks() {
    let task_vect = get_tasks();

    let mut x: usize = 0;
    loop {
        if x == task_vect.len() - 1 {
            break;
        } else {
            println!("Task {}: {}", x + 1, task_vect[x]);
            x += 1;
        }
    }
}
