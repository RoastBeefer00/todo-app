use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use gloo_console::log;
use std::panic;

extern crate console_error_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const STORAGE_KEY: &str = "todos";

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Todos {
            todos: Vec::new()
        }
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    
    fn delete(&mut self, id: Uuid) {
        self.todos.retain(|todo| todo.id != id);
    }

    fn mark_all_complete(&mut self) {
        self.todos
            .iter_mut()
            .for_each(|todo| todo.mark_complete());
    }

    fn mark_all_active(&mut self) {
        self.todos
            .iter_mut()
            .for_each(|todo| todo.mark_active());
    }

    fn toggle_task_complete(&mut self, id: Uuid) {
        self.todos
            .iter_mut()
            .for_each(|todo| {
                if todo.id == id {
                    todo.toggle_complete();
                }
            });
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: Uuid,
    task: String,
    completed: bool,
}

impl Todo {
    fn new(task: String) -> Self {
        Todo {
            id: Uuid::new_v4(),
            task: task,
            completed: false,
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
        log!(self.completed);
    }

    fn mark_active(&mut self) {
        self.completed = false;
    }

    fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

fn get_todos_private() -> Todos {
    let stored: String = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    if stored == "" {
        Todos::new()
    } else {
        let todos: Todos = serde_json::from_str(&stored).unwrap();
        todos
    }
}

fn update_local_storage(todos: &Todos) {
    let todos_serialized = serde_json::to_string(todos).unwrap();
    LocalStorage::set(STORAGE_KEY, &todos_serialized).ok();
}

#[wasm_bindgen(js_name="getTodos")]
pub async fn get_todos() -> JsValue {
    let stored: String = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    log!(stored.clone());
    let todos: Todos = serde_json::from_str(&stored).unwrap();

    serde_wasm_bindgen::to_value(&todos).unwrap()
}

#[wasm_bindgen(js_name="addTodo")]
pub fn add_todo(task: String) -> Todos {
    let todo = Todo::new(task);
    let mut todos = get_todos_private();
    todos.add(todo);
    update_local_storage(&todos);

    todos
}

#[wasm_bindgen(js_name="deleteTodo")]
pub fn delete_todo(id: String) -> Todos {
    let id = Uuid::parse_str(&id).unwrap();
    let mut todos = get_todos_private();
    todos.delete(id);
    update_local_storage(&todos);

    todos
}

#[wasm_bindgen(js_name="markAllComplete")]
pub fn mark_all_complete() -> Todos {
    let mut todos = get_todos_private();
    todos.mark_all_complete();
    update_local_storage(&todos);

    todos
}

#[wasm_bindgen(js_name="markAllActive")]
pub fn mark_all_active() -> Todos {
    let mut todos = get_todos_private();
    todos.mark_all_active();
    update_local_storage(&todos);

    todos
}

#[wasm_bindgen(js_name="toggleComplete")]
pub fn toggle_complete(id: String) -> Todos {
    let id = Uuid::parse_str(&id).unwrap();
    let mut todos = get_todos_private();
    todos.toggle_task_complete(id);
    update_local_storage(&todos);

    todos
}

#[wasm_bindgen(js_name="init")]
pub fn init() {
    let stored: String = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    if stored == "" {
        let todos = Todos::new();
        update_local_storage(&todos);
    }
}
