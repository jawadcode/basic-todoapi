use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type TodoList = Vec<Todo>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub complete: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Search {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub filter: Option<String>,
}

#[derive(Clone)]
pub struct Store {
    pub todolist: Arc<RwLock<TodoList>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            todolist: Arc::new(RwLock::new(Vec::<Todo>::new())),
        }
    }
}
