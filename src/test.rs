 let mut todo_list = TodoList::initialize();

    let todo = Todo {
        name: "Task 1".to_string(),
        title: "First Task".to_string(),
        description: "This is the first task".to_string(),
        is_completed: false,
    };

    todo_list.new(todo);
    println!("{:?}", todo_list);

    let id = 1; // Assuming we know the ID of the task to mark as done

    let updated_desc = "Updated description for Task 1".to_string();
    todo_list.update_task(&id, updated_desc);
    println!("{:?}", todo_list);

    todo_list.mark_as_done(&id);
    println!("{:?}", todo_list);

    todo_list.delete_task(&id);
    println!("{:?}", todo_list);