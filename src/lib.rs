// •	A command-line tool where users can add, mark done, and delete tasks.
// 	•	Teaches: structs, enums, file I/O (serde + JSON/TOML).


#[derive(Debug, Clone)]
pub struct Todo {
    pub name: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

type Id = u32;

#[derive(Debug)]
pub struct TodoList {
    pub data: Vec<(Todo, Id)>,
}

impl TodoList {
    pub fn initialize() -> Self {
        Self { data: Vec::new() }
    }

    pub fn new(&mut self, todo: Todo) -> &Self {
        let new_todo = Todo {
            name: todo.name,
            title: todo.title,
            description: todo.description,
            is_completed: todo.is_completed,
        };

        let id = self.data.iter().map(|(_, id)| *id).max().unwrap_or(0) + 1;
        self.data.push((new_todo, id));

        self
    }

    pub fn mark_as_done(&mut self, id: &Id) -> &Self {
        let todo: Vec<(Todo, u32)> = self
            .data
            .iter_mut()
            .map(|(t, i)| {
                println!("Checking task with ID: {}", &i);
                println!("Checking task with ID again: {}", &id);
                if i == id {
                    let mut t = t.clone();
                    t.is_completed = true;
                    (t, *i)
                } else {
                    (t.clone(), *i)
                }
            })
            .collect::<Vec<_>>();
        self.data = todo;
        self
    }

    pub fn delete_task(&mut self, id: &Id) -> &Self {
        let todo = self
            .data
            .iter()
            .filter(|(_, todo_id)| todo_id != id)
            .cloned()
            .collect::<Vec<_>>();
        self.data = todo;
        self
    }

    pub fn update_task(&mut self, id: &Id, desc: String) -> &Self {
        let todo = self
            .data
            .iter_mut()
            .map(|(t, i)| {
                if i == id {
                    // let mut t = t.clone();
                    t.description = desc.clone();
                    (t.clone(), *i)
                } else {
                    (t.clone(), *i)
                }
            })
            .collect::<Vec<_>>();

        self.data = todo;
        self
    }
}
