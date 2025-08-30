use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    task: String,
    is_completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        main_menu();
        let mut user_input_action = String::new();
        io::stdin()
            .read_line(&mut user_input_action)
            .expect("Failed to read line");

        let mut user_input = user_input_action.trim().parse().expect("Invalid input!");

        match user_input {
            1 => view_todos(&tasks),
            2 => add_todo(&mut tasks),
            3 => edit_todo(&mut tasks),
            4 => break,
            _ => println!("Invalid input, please enter numbers between 1 to 4"),
        };
    }
}

fn main_menu() {
    println!("\n\n\n==============RUST TODO APPLICATION==============");
    println!("Please press numbers between 1 to 4 to perform an action");
    println!("1. View all todos");
    println!("2. Add a todo");
    println!("3. Edit a todo");
    println!("4. Exit");
    println!("\n");
    print!("> ");
    io::stdout().flush().expect("Failed to flush the line!");
}

fn view_todos(todos: &Vec<Task>) {
    // if todos.len() > 0 {
    if !todos.is_empty() {
        println!("Here's a list of tasks to do...");
        for (index, task) in todos.iter().enumerate() {
            println!(
                "Task {index}: [ {} ] {}",
                if task.is_completed { "✅" } else { "" },
                task.task
            );
        }
    } else {
        println!("Wohooo, there's no pending task to do...")
    }
}

fn add_todo(tasks: &mut Vec<Task>) {
    println!("Add a task to do...");
    print!("> ");
    io::stdout().flush().expect("Failed to flush the line!");
    let mut task_input = String::new();
    io::stdin()
        .read_line(&mut task_input)
        .expect("Failed to read line");

    tasks.push(Task {
        task: task_input,
        is_completed: false,
    })
}

fn edit_todo(tasks: &mut Vec<Task>) {
    println!("Enter the task no, to update the status");
    print!("> ");
    io::stdout().flush().expect("Failed to flush the line!");
    let mut task_input = String::new();
    io::stdin()
        .read_line(&mut task_input)
        .expect("Failed to read line");

    match task_input.trim().parse::<usize>() {
        Ok(num) => {
            if tasks.len() == 0 {
                println!("Task does not present in the list, Please try again!");
                return;
            }

            tasks[num].is_completed = !tasks[num].is_completed;
        }
        Err(err) => {
            println!("Error");
            return;
        }
    }
}
