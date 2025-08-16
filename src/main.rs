use cli_todo_list_manager::{Todo, TodoList};
use std::env;
use std::env::Args;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum TodoActions {
    Initialize,
    CreateNew,
    Update,
    Delete,
    MarkAsCompleted,
    Exit,
    Export,
}

#[derive(Debug)]
struct ArgumentConfig {
    action: TodoActions,
    additional_args: String,
}

fn build_config(args: Args) -> ArgumentConfig {
    let mut action = TodoActions::Initialize;
    let mut additional_args = String::new();

    for arg in args {
        match arg.as_str() {
            "initialize" => action = TodoActions::Initialize,
            "create" => action = TodoActions::CreateNew,
            "update" => action = TodoActions::Update,
            "delete" => action = TodoActions::Delete,
            "mark_completed" => action = TodoActions::MarkAsCompleted,
            "exit" => action = TodoActions::Exit,
            "export" => action = TodoActions::Export,
            _ => additional_args.push_str(&arg),
        }
    }

    ArgumentConfig {
        action,
        additional_args,
    }
}

fn export_todo_list_to_file(todo_list: &TodoList) -> io::Result<()> {
    let file_path = "todo_list.txt";
    let mut file = std::fs::File::create(file_path)?;

    for (todo, id) in &todo_list.data {
        writeln!(
            file,
            "ID: {}, Name: {}, Title: {}, Description: {}, Completed: {}",
            id, todo.name, todo.title, todo.description, todo.is_completed
        )?;
    }

    println!("Todo list exported to {}", file_path);
    Ok(())
}

fn match_input_to_action(input: &str) -> TodoActions {
    match input {
        "1" => TodoActions::Initialize,
        "2" => TodoActions::CreateNew,
        "3" => TodoActions::Update,
        "4" => TodoActions::Delete,
        "5" => TodoActions::MarkAsCompleted,
        "6" => TodoActions::Exit,
        "7" => TodoActions::Export,

        _ => TodoActions::Initialize, // Default action
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    println!("waiting for input...");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn use_cli() {
    let arg = env::args();

    let config = build_config(arg);

    let mut todo_list = TodoList::initialize();

    loop {
        println!(
            "=======STARTING TODO LIST=======\n Select an action: \n 1. Initialize \n 2. Create New Task \n 3. Update Task \n 4. Delete Task \n 5. Mark Task as Completed \n 6. Exit"
        );

        let input = get_user_input();

        match match_input_to_action(&input) {
            TodoActions::Initialize => {
                println!("{:?}", todo_list);
            }
            TodoActions::CreateNew => {
                println!("Creating a new task...");
                println!("Please enter task name:");
                let name = get_user_input();
                println!("Please enter task title:");
                let title = get_user_input();
                println!("Please enter task description:");
                let description = get_user_input();

                let todo = Todo {
                    name,
                    title,
                    description,
                    is_completed: false,
                };
                todo_list.new(todo);
                println!("{:?}", todo_list);
            }
            TodoActions::Update => {
                println!("Updating an existing task...");
                println!("Enter todo id");
                let id = get_user_input()
                    .parse::<u32>()
                    .expect("Please enter a valid ID");
                println!("Updating task with ID: {}", id);
                println!("Please enter new description:");
                let updated_desc = get_user_input();
                todo_list.update_task(&id, updated_desc);
                println!("{:?}", todo_list);
            }
            TodoActions::Delete => {
                println!("Deleting a task...");
                println!("Please enter the ID of the task to delete:");
                let id = get_user_input()
                    .parse::<u32>()
                    .expect("Please enter a valid ID");
                println!("Deleting task with ID: {}", id);
                todo_list.delete_task(&id);
                println!("{:?}", todo_list);
            }
            TodoActions::MarkAsCompleted => {
                println!("Marking a task as completed...");
                println!("Please enter the ID of the task to mark as completed:");
                let id = get_user_input()
                    .parse::<u32>()
                    .expect("Please enter a valid ID");
                todo_list.mark_as_done(&id);
                println!("{:?}", todo_list);
            }
            TodoActions::Exit => {
                println!("Exiting the application.");
                break;
            }
            TodoActions::Export => {
                if let Err(e) = export_todo_list_to_file(&todo_list) {
                    eprintln!("Error exporting todo list: {}", e);
                }
            }
        }
    }

    println!("=======END OF TODO LIST=======");
}

fn main() {
    use_cli();
}
