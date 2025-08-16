# CLI Todo List Manager

A command-line interface (CLI) application built in Rust for managing your daily tasks and todo items.

## Features

- Create new tasks with name, title, and description
- Update existing tasks
- Delete tasks
- Mark tasks as completed
- Initialize/reset todo list
- Interactive command-line interface

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Eminivince/cli_todo_list_manager.git
cd cli_todo_list_manager
```

2. Build the project:

```bash
cargo build
```

3. Run the application:

```bash
cargo run
```

## Usage

The application provides an interactive menu with the following options:

1. Initialize - Reset/show current todo list
2. Create New Task - Add a new task to the list
3. Update Task - Modify an existing task
4. Delete Task - Remove a task from the list
5. Mark Task as Completed - Toggle task completion status
6. Exit - Close the application

### Example Commands

When prompted, enter the number corresponding to the action you want to perform:

- Enter `1` to view/initialize the todo list
- Enter `2` to create a new task
- Enter `3` to update an existing task
- Enter `4` to delete a task
- Enter `5` to mark a task as completed
- Enter `6` or type `exit` to quit the application

## Project Structure

- `src/main.rs` - Main application logic and CLI interface
- `src/lib.rs` - Core functionality and data structures

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
