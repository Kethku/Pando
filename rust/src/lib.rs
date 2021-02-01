#[macro_use]
extern crate nom;

mod parser;
mod generator;
mod operations;

use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::console;

use parser::{Todo, parse_pando};
use generator::{generate_dot_file, generate_pando_file};
use operations::*;

#[derive(Serialize)]
struct CompileResult {
    success: bool,
    dotCode: Option<String>,
    reason: Option<String>
}

#[wasm_bindgen]
pub fn compile(pando_code: &str) -> JsValue {
    ::std::panic::set_hook(Box::new(|info| {
        console::error_1(&JsValue::from_str(&format!("!!! RUST PANIC !!! {:?}", info)));
    }));

    match generate_dot_file(parse_pando(pando_code)) {
        Ok(dot_code) => JsValue::from_serde(&CompileResult {
            success: true, 
            dotCode: Some(dot_code),
            reason: None
        }),
        Err(reason) => JsValue::from_serde(&CompileResult {
            success: false,
            dotCode: None,
            reason: Some(reason)
        })
    }.unwrap()
}

fn pando_operation(pando_code: &str, operation: impl Fn(Vec<Todo>) -> Vec<Todo>) -> String {
    generate_pando_file(operation(parse_pando(pando_code)))
}

#[wasm_bindgen]
pub fn newTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| new_task_operation(task_identifier, tasks))
}

#[wasm_bindgen]
pub fn deleteTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| delete_task_operation(task_identifier, tasks))
}

#[wasm_bindgen]
pub fn toggleDependency(task_identifier: &str, dependency_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| toggle_dependency_operation(task_identifier, dependency_identifier, tasks))
}

#[wasm_bindgen]
pub fn progressTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| progress_task_operation(task_identifier, tasks))
}

#[cfg(test)]
mod tests;
