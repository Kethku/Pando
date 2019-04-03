#![feature(custom_attribute)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate stdweb;

mod parser;
mod generator;
mod operations;

use stdweb::Value;

use parser::{Todo, parse_pando};
use generator::{generate_dot_file, generate_pando_file};
use operations::*;

#[js_export]
pub fn compile(pando_code: &str) -> Value {
    ::std::panic::set_hook(Box::new(|info| {
        console!(error, format!("!!! RUST PANIC !!! {:?}", info));
    }));

    match generate_dot_file(parse_pando(pando_code)) {
        Ok(dot_code) => js! {
            return { success: true, dotCode: @{dot_code} }
        },
        Err(reason) => js! {
            return { success: false, reason: @{reason} }
        }
    }
}

fn pando_operation(pando_code: &str, operation: impl Fn(Vec<Todo>) -> Vec<Todo>) -> String {
    generate_pando_file(operation(parse_pando(pando_code)))
}

#[js_export]
pub fn newTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| new_task_operation(task_identifier, tasks))
}

#[js_export]
pub fn deleteTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| delete_task_operation(task_identifier, tasks))
}

#[js_export]
pub fn toggleDependency(task_identifier: &str, dependency_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| toggle_dependency_operation(task_identifier, dependency_identifier, tasks))
}

#[js_export]
pub fn progressTask(task_identifier: &str, pando_code: &str) -> String {
    pando_operation(pando_code, |tasks| progress_task_operation(task_identifier, tasks))
}

#[cfg(test)]
mod tests;
