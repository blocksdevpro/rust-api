use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct TodoJson {
    pub id: u16,
    pub title: String,
    pub completed: bool,
}

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

    pub fn get_todos(&self) -> Vec<TodoJson> {
        return self.todos.values().cloned().collect();
    }

    #[warn(dead_code)]
    pub fn get_todo(&self, id: u16) -> Option<TodoJson> {
        return self.todos.get(&id).cloned();
    }

    pub fn create_todo(&mut self, title: &str) -> TodoJson {
        let id = self.next_id;
        let todo = TodoJson {
            id,
            title: title.to_string(),
            completed: false,
        };

        self.todos.insert(id, todo.clone());
        self.next_id += 1;
        return todo;
    }

    #[warn(unused)]
    pub fn update_todo(
        &mut self,
        id: u16,
        title: Option<&str>,
        completed: Option<bool>,
    ) -> Option<TodoJson> {
        let todo = self.todos.get_mut(&id)?;

        if let Some(title) = title {
            todo.title = title.to_string();
        }
        if let Some(completed) = completed {
            todo.completed = completed;
        }

        Some(todo.clone())
    }
}
