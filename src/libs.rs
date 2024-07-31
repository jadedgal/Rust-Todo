use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn output_tasks() -> Result<(), io::Error> {
    match get_tasks() {
        Err(e) => Err(e),
        Ok(None) => {
            println!("No tasks to display");
            Ok(())
        }
        Ok(Some(tasks)) => {
            if tasks.is_empty() {
                println!("No tasks to display");
            } else {
                for (index, task) in tasks.iter().enumerate() {
                    println!("{}. {}", index + 1, task);
                }
            }
            Ok(())
        }
    }
}

pub fn get_tasks() -> Result<Option<Vec<String>>, io::Error> {
    let file_path = Path::new("tasks.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)?;

    let mut tasks = String::new();
    file.read_to_string(&mut tasks)?;

    if tasks.trim().is_empty() {
        Ok(None)
    } else {
        let task_vect: Vec<String> = tasks.lines().map(|s| s.to_string()).collect();
        Ok(Some(task_vect))
    }
}

pub fn add_task(task: Option<String>) -> Result<(), io::Error> {
    let task = match task {
        Some(t) => t,
        None => {
            let mut task = String::new();
            println!("Enter task to add:");
            io::stdin().read_line(&mut task)?;
            task
        }
    };
    let mut tasks = get_tasks()?.unwrap_or_default();
    tasks.push(task.trim().to_string());
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")?;
    file.write_all(tasks.join("\n").as_bytes())?;
    Ok(())
}

pub fn remove_tasks(tasks: Option<Vec<usize>>) -> Result<(), io::Error> {
    let tasks_to_remove = match tasks {
        None => {
            let mut number = String::new();
            println!("Enter task number to remove:");
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read from stdin");
            let number: usize = match number.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Please enter a valid number!");
                    return Ok(());
                }
            };
            vec![number]
        }
        Some(t) => t,
    };

    let mut tasks = match get_tasks()? {
        Some(t) => t,
        None => vec![],
    };

    let mut tasks_to_remove = tasks_to_remove;
    tasks_to_remove.sort_unstable();
    tasks_to_remove.dedup();

    for task_index in tasks_to_remove.iter().rev() {
        if *task_index > 0 && *task_index <= tasks.len() {
            tasks.remove(task_index - 1);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Task number {} is out of bounds", task_index),
            ));
        }
    }

    let mut file = File::create("tasks.txt")?;
    file.write_all(tasks.join("\n").as_bytes())?;
    Ok(())
}

pub fn add(arguments: &[String]) -> Result<(), io::Error> {
    if arguments.len() >= 3 {
        let task = arguments[2..].join(" ");
        add_task(Some(task))
    } else {
        add_task(None)
    }
    .map(|_| println!("Task added successfully!"))
    .map_err(|e| {
        println!("Error adding task: {}", e);
        e
    })
}

pub fn remove(arguments: &[String]) -> Result<(), io::Error> {
    let indices: Result<Vec<usize>, _> =
        arguments[2..].iter().map(|s| s.parse::<usize>()).collect();

    match indices {
        Ok(indices) => {
            match remove_tasks(Some(indices)) {
                Ok(_) => {
                    println!("Task(s) removed successfully!");
                    Ok(())
                }
                Err(e) => {
                    println!("Error removing task: {}", e);
                    Ok(()) // Change this to Ok(()) to prevent error propagation
                }
            }
        }
        Err(e) => {
            println!("Error parsing indices: {}", e);
            Ok(())
        }
    }
}
