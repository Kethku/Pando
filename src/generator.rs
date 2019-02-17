use crate::parser::*;
use crate::abbreviations::*;

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
        1 => Some(dep_variables.first().unwrap().to_string()),
        _ => Some(format!("{{ {} }}", dep_variables.join(" ")))
    }
}

pub fn generate_dependencies(tasks: &Vec<Todo>) -> String {
    let dependent_task_lookup = resolve_dependent_tasks(&tasks);
    let entries: Vec<String> = tasks.iter().filter_map(|task| {
        let dependent_tasks = dependent_task_lookup[&task.identifier].clone();
        let task_variable = generate_identifier_variable(&task.identifier);
        generate_dependency_list(dependent_tasks)
            .map(|deps| format!("{} -> {};", task_variable, deps))
    }).collect();

    entries.join("\n  ")
}

pub fn generate_dot_file(tasks: Vec<Todo>) -> String {
    let definitions = generate_definitions(&tasks);
    let dependencies = generate_dependencies(&tasks);

    format!(
"digraph {{
  node [shape=record, splines=\"curve\"];
  {}

  {}
}}", definitions, dependencies)
}
