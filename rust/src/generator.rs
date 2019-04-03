use std::collections::HashMap;
use crate::parser::*;

pub fn generate_task_formatting(task: &Todo) -> String {
    match task.status {
        TaskStatus::Completed =>
            format!("<<font color='gray'>{} <br/> <i>Complete</i></font>>, color=\"gray\"", task.identifier),
        TaskStatus::InProgress =>
            format!("<{} <br/> <i>In Progress</i>>", task.identifier),
        TaskStatus::Waiting =>
            format!("<{}>", task.identifier)
    }
}

pub fn generate_identifier_variable(identifier: &String) -> String {
    identifier.replace(" ", "")
}

pub fn generate_task_definition(task: &Todo) -> String {
    format!("{}[label={}];", generate_identifier_variable(&task.identifier), generate_task_formatting(task))
}

pub fn generate_definitions(tasks: &Vec<Todo>) -> String {
    let definition_entries: Vec<String> = tasks
        .iter()
        .map(generate_task_definition)
        .collect();
    definition_entries.join("\n  ")
}

pub fn generate_dependency_list(deps: Vec<String>) -> Option<String> {
    let dep_variables: Vec<String> = deps
        .iter()
        .map(generate_identifier_variable)
        .collect();

    match dep_variables.len() {
        0 => None,
        1 => Some(dep_variables.first()?.to_string()),
        _ => Some(format!("{{ {} }}", dep_variables.join(" ")))
    }
}

pub fn resolve_dependent_tasks(tasks: &Vec<Todo>) -> Result<HashMap<String, Vec<String>>, String> {
    let mut dependent_tasks = HashMap::new();

    for task in tasks.iter() {
        dependent_tasks.insert(task.identifier.clone(), Vec::new());
    }

    for task in tasks.iter() {
        for dependency in task.dependencies.clone() {
            match tasks.iter().find(|task| &task.identifier == &dependency) {
                Some(_) =>
                    dependent_tasks
                        .entry(dependency.clone())
                        .and_modify(|dependent_tasks| dependent_tasks.push(task.identifier.clone())),
                None => return Err(format!("Could not find dependency: {}", dependency))
            };
        }
    }

    Ok(dependent_tasks)
}

pub fn generate_dependencies(tasks: &Vec<Todo>) -> Result<String, String> {
    let dependent_task_lookup = resolve_dependent_tasks(&tasks)?;
    let entries: Vec<String> = tasks.iter().filter_map(|task| {
        let dependent_tasks = dependent_task_lookup[&task.identifier].clone();
        let task_variable = generate_identifier_variable(&task.identifier);
        generate_dependency_list(dependent_tasks)
            .map(|deps| format!("{} -> {};", task_variable, deps))
    }).collect();

    Ok(entries.join("\n  "))
}

pub fn generate_dot_file(tasks: Vec<Todo>) -> Result<String, String> {
    let definitions = generate_definitions(&tasks);
    let dependencies = generate_dependencies(&tasks)?;

    Ok(format!(
"digraph {{
  node [shape=record, splines=\"curve\"];
  {}

  {}
}}", definitions, dependencies))
}

pub fn generate_pando_status(status: TaskStatus) -> String {
    (match status {
        TaskStatus::Completed => "x ",
        TaskStatus::InProgress => "> ",
        TaskStatus::Waiting => "- "
    }).to_owned()
}

pub fn generate_pando_dependencies(dependencies: Vec<String>) -> String {
    if dependencies.is_empty() {
        String::new()
    } else {
        format!(" [ {} ]", dependencies.join(", "))
    }
}

pub fn generate_pando_file(tasks: Vec<Todo>) -> String {
    let task_lines: Vec<String> = tasks.into_iter().map(|task| {
        generate_pando_status(task.status) +
        &task.identifier +
        &generate_pando_dependencies(task.dependencies)
    }).collect();

    task_lines.join("\n")
}
