use crate::parser::*;

fn modify_task(task_identifier: &str, tasks: Vec<Todo>, operation: impl Fn(Todo) -> Option<Todo>) -> Vec<Todo> {
    tasks
        .into_iter()
        .filter_map(|todo| {
            if todo.identifier == task_identifier {
                operation(todo)
            } else {
                Some(todo)
            }
        }).collect()
}

pub fn toggle_dependency_operation(task_identifier: &str, dependency_identifier: &str, tasks: Vec<Todo>) -> Vec<Todo> {
    modify_task(task_identifier, tasks, |todo| {
        if todo.dependencies.iter().all(|dependency| dependency != dependency_identifier) {
            let mut new_dependencies = todo.dependencies.clone();
            new_dependencies.push(dependency_identifier.to_string());
            Some(Todo { dependencies: new_dependencies, ..todo })
        } else {
            let new_dependencies = todo.dependencies
                .into_iter()
                .filter(|dependency| dependency != dependency_identifier)
                .collect();
            Some(Todo { dependencies: new_dependencies, ..todo })
        }
    })
}

pub fn progress_task_operation(task_identifier: &str, tasks: Vec<Todo>) -> Vec<Todo> {
    modify_task(task_identifier, tasks, |todo| Some(Todo { status: todo.status.next(), ..todo }))
}

pub fn new_task_operation(task_identifier: &str, mut tasks: Vec<Todo>) -> Vec<Todo> {
    tasks.push(Todo::new(TaskStatus::Waiting, task_identifier, Vec::new()));
    tasks
}

pub fn delete_task_operation(task_identifier: &str, tasks: Vec<Todo>) -> Vec<Todo> {
    tasks
        .into_iter()
        .filter_map(|todo| {
            if todo.identifier == task_identifier {
                None
            } else {
                let new_dependencies = todo.dependencies
                    .into_iter()
                    .filter(|dependency| dependency != task_identifier)
                    .collect();
                Some(Todo { dependencies: new_dependencies, ..todo })
            }
        }).collect()
}
