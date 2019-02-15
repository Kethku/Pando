use crate::parser::*;

use std::collections::HashMap;

fn graph_wrapper(contents: String) -> String {
    format!(
        "digraph {{
  node [shape = record, splines=\"curve\"];
  {}
}}", contents)
}

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
