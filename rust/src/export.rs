use crate::parser::*;

pub fn remove_dependency(task_identifier: &str, dependency_identifier: &str, tasks: Vec<Todo>) -> Vec<Todo> {
    tasks
        .into_iter()
        .map(|todo| {
            if todo.identifier == task_identifier {
                let new_dependencies = todo.dependencies
                    .into_iter()
                    .filter(|dependency| dependency != dependency_identifier)
                    .collect();
                Todo { dependencies: new_dependencies, ..todo }
            } else {
                todo
            }
        })
        .collect();
}
