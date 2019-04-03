use super::parser::*;
use super::generator::generate_dot_file;
use super::compile;

use nom::types::CompleteStr;

#[test]
fn parse_status_works() {
    assert_eq!(parse_status(CompleteStr("x")), Ok((CompleteStr(""), TaskStatus::Completed)));
    assert_eq!(parse_status(CompleteStr(">")), Ok((CompleteStr(""), TaskStatus::InProgress)));
    assert_eq!(parse_status(CompleteStr("-")), Ok((CompleteStr(""), TaskStatus::Waiting)));
}

#[test]
fn parse_identifier_works() {
    assert_eq!(parse_identifier(CompleteStr(" Implement Parser\n")), Ok((CompleteStr("\n"), "Implement Parser")));
}

#[test]
fn parse_dependencies_works() {
    assert_eq!(parse_dependencies(CompleteStr("[ Implement Parser, DOT Generator, Brainstorm ]")),
               Ok((CompleteStr(""), vec!["Implement Parser", "DOT Generator", "Brainstorm"])));
    assert_eq!(parse_dependencies(CompleteStr("")), Ok((CompleteStr(""), vec![])));
}

#[test]
fn parse_todo_works() {
    assert_eq!(
        parse_todo(CompleteStr("> Implement Parser [ IP, DG, B ]")),
        Ok((CompleteStr(""), Todo::new(TaskStatus::InProgress, "Implement Parser", vec!["IP", "DG", "B"])))
    );

    assert_eq!(
        parse_todo(CompleteStr("x Brainstorm\n")),
        Ok((CompleteStr(""), Todo::new(TaskStatus::Completed, "Brainstorm", vec![])))
    );
}

#[test]
fn parse_pando_works() {
    assert_eq!(
        parse_pando(
"x Brainstorm
> Specify Format [ Brainstorm ]
- Implement Parser [ Specify Format ]
- DOT Generator [ Brainstorm ]
- Command Line [ Implement Parser, DOT Generator ]"),
        vec![
            Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
            Todo::new(TaskStatus::InProgress, "Specify Format", vec!["Brainstorm"]),
            Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["Specify Format"]),
            Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["Brainstorm"]),
            Todo::new(TaskStatus::Waiting, "Command Line", vec!["Implement Parser", "DOT Generator"])
        ]);
}

#[test]
fn generator_dot_file_works() {
    let generated = generate_dot_file(vec![
        Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
        Todo::new(TaskStatus::InProgress, "Specify Format", vec!["Brainstorm"]),
        Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["Specify Format"]),
        Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["Brainstorm"]),
        Todo::new(TaskStatus::Waiting, "Command Line", vec!["Implement Parser", "DOT Generator"])
    ]);

    let expected = "digraph {
  node [shape=record, splines=\"curve\"];
  Brainstorm[label=<<font color='gray'>Brainstorm <br/> <i>Complete</i></font>>, color=\"gray\"];
  SpecifyFormat[label=<Specify Format <br/> <i>In Progress</i>>];
  ImplementParser[label=<Implement Parser>];
  DOTGenerator[label=<DOT Generator>];
  CommandLine[label=<Command Line>];

  Brainstorm -> { SpecifyFormat DOTGenerator };
  SpecifyFormat -> ImplementParser;
  ImplementParser -> CommandLine;
  DOTGenerator -> CommandLine;
}".to_string();

    assert_eq!(generated, expected);
}

#[test]
fn compile_works() {
    let generated = compile(
"x Brainstorm
> Specify Format [ Brainstorm ]
- Implement Parser [ Specify Format ]
- DOT Generator [ Brainstorm ]
- Command Line [ Implement Parser, DOT Generator ]");

    let expected = "digraph {
  node [shape=record, splines=\"curve\"];
  Brainstorm[label=<<font color='gray'>Brainstorm <br/> <i>Complete</i></font>>, color=\"gray\"];
  SpecifyFormat[label=<Specify Format <br/> <i>In Progress</i>>];
  ImplementParser[label=<Implement Parser>];
  DOTGenerator[label=<DOT Generator>];
  CommandLine[label=<Command Line>];

  Brainstorm -> { SpecifyFormat DOTGenerator };
  SpecifyFormat -> ImplementParser;
  ImplementParser -> CommandLine;
  DOTGenerator -> CommandLine;
}".to_string();

    assert_eq!(generated, expected);
}
