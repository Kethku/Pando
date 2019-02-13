#[macro_use]
extern crate nom;

use std::str;
use nom::types::CompleteStr;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Completed,
    InProgress,
    Waiting
}

#[derive(Debug, PartialEq)]
struct Todo {
    pub status: TaskStatus,
    pub identifier: String,
    pub dependencies: Vec<String>
}

impl Todo {
    fn new(status: TaskStatus, identifier: &str, dependencies: Vec<&str>) -> Todo {
        Todo {
            status,
            identifier: identifier.to_string(),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect()
        }
    }
}

named!(parse_status<CompleteStr, TaskStatus>,
    alt!(map!(tag!("x"), |_| TaskStatus::Completed) |
         map!(tag!(">"), |_| TaskStatus::InProgress) |
         map!(tag!("-"), |_| TaskStatus::Waiting)));

named!(parse_identifier<CompleteStr, &str>, map!(is_not!("[\n"), |id| id.trim()));

named!(parse_dependencies<CompleteStr, Vec<&str> >,
    map!(opt!(complete!(delimited!(
        char!('['),
        separated_list!(char!(','), map!(is_not!(",]"), |dep| dep.trim())),
        char!(']')))),
    |deps| deps.unwrap_or(Vec::new())));

named!(todo<CompleteStr, Todo>, terminated!(
    do_parse!(
        status: parse_status >>
        identifier: parse_identifier >>
        dependencies: parse_dependencies >>
        (Todo::new(status, identifier, dependencies))
    ),
alt!(eof!() | nom::eol)));

fn parse_pando(input: &str) -> Option<Vec<Todo>> {
    match many1!(CompleteStr(input), todo) {
        Ok((_, todos)) => Some(todos),
        Err(_) => None
    }
}

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
            format!("<<font color='gray'>{} <br/> <i>Complete</i></font>>, color=\"gray\"",
                    task.identifier),
        TaskStatus::InProgress =>
            format!("<{} <br/> <i>In Progress</i>>",
                    task.identifier),
        TaskStatus::Waiting =>
            format!("<{}>",
                    task.identifier)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_works() {
        assert_eq!(parse_status(CompleteStr("x")), Ok((CompleteStr(""), TaskStatus::Completed)));
        assert_eq!(parse_status(CompleteStr(">")), Ok((CompleteStr(""), TaskStatus::InProgress)));
        assert_eq!(parse_status(CompleteStr("-")), Ok((CompleteStr(""), TaskStatus::Waiting)));
    }

    #[test]
    fn identifier_works() {
        assert_eq!(parse_identifier(CompleteStr(" Implement Parser\n")), Ok((CompleteStr("\n"), "Implement Parser")));
    }

    #[test]
    fn dependencies_works() {
        assert_eq!(parse_dependencies(CompleteStr("[ IP, DG, B ]")), Ok((CompleteStr(""), vec!["IP", "DG", "B"])));
        assert_eq!(parse_dependencies(CompleteStr("")), Ok((CompleteStr(""), vec![])));
    }

    #[test]
    fn todo_works() {
        assert_eq!(
            todo(CompleteStr("> Implement Parser [ IP, DG, B ]")),
            Ok((CompleteStr(""), Todo::new(TaskStatus::InProgress, "Implement Parser", vec!["IP", "DG", "B"])))
        );

        assert_eq!(
            todo(CompleteStr("x Brainstorm\n")),
            Ok((CompleteStr(""), Todo::new(TaskStatus::Completed, "Brainstorm", vec![])))
        );
    }

    #[test]
    fn pando_works() {
        assert_eq!(
            parse_pando(
"x Brainstorm
> Specify Format [ B ]
- Implement Parser [ SF ]
- DOT Generator [ B ]
- Command Line [ IP, DG ]"),
            Some(vec![
                Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
                Todo::new(TaskStatus::InProgress, "Specify Format", vec!["B"]),
                Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["SF"]),
                Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["B"]),
                Todo::new(TaskStatus::Waiting, "Command Line", vec!["IP", "DG"])
            ]));
    }
}
