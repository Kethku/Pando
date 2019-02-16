use crate::parser::*;

use std::collections::HashMap;

fn task_formatting(task: Todo) -> String {
    match task.status {
        TaskStatus::Completed =>
            format!("<<font color='gray'>{} <br/> <i>Complete</i></font>>, color=\"gray\"", task.identifier),
        TaskStatus::InProgress =>
            format!("<{} <br/> <i>In Progress</i>>", task.identifier),
        TaskStatus::Waiting =>
            format!("<{}>", task.identifier)
    }
}

fn identifier_variable(identifier: String) -> String {
    identifier.replace(" ", "")
}

fn task_definition(task: Todo) -> String {
    format!("{}[label={}];", identifier_variable(task.identifier), task_formatting(task))
}

fn definitions(tasks: Vec<Todo>) -> String {
    tasks.iter().map(task_definition).join('\n')
}

fn dependency_list(deps: Vec<String>) -> Option<String> {
    match deps.len() {
        0 => None,
        1 => Some(deps.first()),
        _ => Some(format!("{{ {} }}", deps.join(" ")))
    }
}

fn dependencies(tasks: Vec<Todo>) -> String {
    let dependent_task_lookup = resolve_dependent_tasks(tasks);
}

fn generate_output(tasks: Vec<Todo>) -> String {
    format!(
        "digraph {{
  node [shape = record, splines=\"curve\"];
  {definitions}

  {dependencies}
}}", )
}
