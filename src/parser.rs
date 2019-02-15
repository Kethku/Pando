use nom::types::CompleteStr;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Completed,
    InProgress,
    Waiting
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Todo {
    pub status: TaskStatus,
    pub identifier: String,
    pub dependencies: Vec<String>
}

impl Todo {
    pub fn new(status: TaskStatus, identifier: &str, dependencies: Vec<&str>) -> Todo {
        Todo {
            status,
            identifier: identifier.to_string(),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect()
        }
    }
}

named!(pub parse_status<CompleteStr, TaskStatus>,
    alt!(map!(tag!("x"), |_| TaskStatus::Completed) |
         map!(tag!(">"), |_| TaskStatus::InProgress) |
         map!(tag!("-"), |_| TaskStatus::Waiting)));

named!(pub parse_identifier<CompleteStr, &str>, map!(is_not!("[\n"), |id| id.trim()));

named!(pub parse_dependencies<CompleteStr, Vec<&str> >,
    map!(opt!(complete!(delimited!(
        char!('['),
        separated_list!(char!(','), map!(is_not!(",]"), |dep| dep.trim())),
        char!(']')))),
    |deps| deps.unwrap_or(Vec::new())));

named!(pub parse_todo<CompleteStr, Todo>, terminated!(
    do_parse!(
        status: parse_status >>
        identifier: parse_identifier >>
        dependencies: parse_dependencies >>
        (Todo::new(status, identifier, dependencies))
    ),
alt!(eof!() | nom::eol)));

pub fn parse_pando(input: &str) -> Option<Vec<Todo>> {
    match many1!(CompleteStr(input), parse_todo) {
        Ok((_, todos)) => Some(todos),
        Err(_) => None
    }
}
