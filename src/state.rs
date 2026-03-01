use crate::models::TodoJson;
use std::collections::HashMap;

pub struct AppState {
    next_id: u16,
    todos: HashMap<u16, TodoJson>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            todos: HashMap::new(),
        }
    }

    pub fn get_todos(&self) -> Vec<&TodoJson> {
        self.todos.values().collect()
    }

    #[allow(dead_code)]
    pub fn get_todo(&self, id: u16) -> Option<&TodoJson> {
        self.todos.get(&id)
    }

    pub fn create_todo(&mut self, title: &str) -> Option<&TodoJson> {
        let id = self.next_id;

        self.todos.insert(
            id,
            TodoJson {
                id,
                title: title.to_string(),
                completed: false,
            },
        );

        self.next_id += 1;
        self.todos.get(&id)
    }

    #[allow(unused)]
    pub fn update_todo(
        &mut self,
        id: u16,
        title: Option<&str>,
        completed: Option<bool>,
    ) -> Option<&TodoJson> {
        let todo = self.todos.get_mut(&id)?;

        if let Some(title) = title {
            todo.title = title.to_string();
        }
        if let Some(completed) = completed {
            todo.completed = completed;
        }

        self.todos.get(&id)
    }

    #[allow(unused)]
    pub fn delete_todo(&mut self, id: u16) -> Option<TodoJson> {
        self.todos.remove(&id)
    }
}
