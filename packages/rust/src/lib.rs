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

#[wasm_bindgen(js_name="getTodos")]
pub async fn get_todos() -> JsValue {
    // log!(storage.clone());
    // match storage {
    //     Some(todos) => {
    //         let todos: Todos = serde_json::from_str(&todos).unwrap();
    //         todos
    //     },
    //     None => Todos::new()
    // }
    let stored: String = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    log!(stored.clone());
    let todos: Todos = serde_json::from_str(&stored).unwrap();
    serde_wasm_bindgen::to_value(&todos).unwrap()
    // let value = local_storage.get_item(STORAGE_KEY).unwrap();
    // match value {
    //    None => Todos::new(),
    //    Some(json) => {
    //        let todos: Todos = serde_json::from_str(&json).unwrap();
    //        todos
    //     },
    // }
    // match local_storage {
    //     Ok(_) => {
    //         let mut todos = Todos::new();
    //         todos.add(Todo::new(String::from("walk the dogs")));
    //         todos
    //     },
    //     _ => Todos::new(),
    // }
}

#[wasm_bindgen(js_name="addTodo")]
pub fn add_todo(task: String) -> Todos {
    let mut todos = get_todos_private();
    let todo = Todo::new(task);
    todos.todos.push(todo);
    let todos_serialized = serde_json::to_string(&todos).unwrap();
    LocalStorage::set(STORAGE_KEY, &todos_serialized).ok();
    todos
   // let mut todos = get_todos();
   // let todo = Todo::new(todos.todos.len() as i32, task); 
   // todos.add(todo);
   // let todos_serialized = serde_json::to_string(&todos).unwrap();
   // web_local_storage_api::set_item("todos", todos_serialized.as_str()).unwrap();
}

#[wasm_bindgen(js_name="deleteTodo")]
pub fn delete_todo(id: String) -> Todos {
    let id = Uuid::parse_str(&id).unwrap();
    let mut todos = get_todos_private();
    todos.todos.retain(|todo| todo.id != id);
    let todos_serialized = serde_json::to_string(&todos).unwrap();
    LocalStorage::set(STORAGE_KEY, &todos_serialized).ok();
    todos
   // let mut todos = get_todos();
   // let todo = Todo::new(todos.todos.len() as i32, task); 
   // todos.add(todo);
   // let todos_serialized = serde_json::to_string(&todos).unwrap();
   // web_local_storage_api::set_item("todos", todos_serialized.as_str()).unwrap();
}

#[wasm_bindgen(js_name="init")]
pub fn init() {
    let stored: String = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    if stored == "" {
        let todos = Todos::new();
        let todos_serialized = serde_json::to_string(&todos).unwrap();
        LocalStorage::set(STORAGE_KEY, &todos_serialized).ok();
    }
}
