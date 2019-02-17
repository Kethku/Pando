use super::parser::*;
use super::abbreviations::*;
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
    assert_eq!(parse_dependencies(CompleteStr("[ IP, DG, B ]")), Ok((CompleteStr(""), vec!["IP", "DG", "B"])));
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
> Specify Format [ B ]
- Implement Parser [ SF ]
- DOT Generator [ B ]
- Command Line [ IP, DG ]"),
        vec![
            Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
            Todo::new(TaskStatus::InProgress, "Specify Format", vec!["B"]),
            Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["SF"]),
            Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["B"]),
            Todo::new(TaskStatus::Waiting, "Command Line", vec!["IP", "DG"])
        ]);
}

#[test]
fn identifier_split_works() {
    assert_eq!(split_identifier("Specify Format"), vec!["specify", "format"]);
    assert_eq!(split_identifier("DOJ Appointment"), vec!["doj", "appointment"]);
}

#[test]
fn abbreviation_split_works() {
    assert_eq!(split_abbreviation("DotA"), vec!["dot", "a"]);
}

#[test]
fn abbreviation_matches_works() {
    assert!(abbreviation_matches("Implement Parser", "IP"));
    assert!(!abbreviation_matches("DOT Generator", "IP"));
    assert!(abbreviation_matches("DOJ Appointment", "DojA"));
    assert!(!abbreviation_matches("DOJ Appointment", "Doj"));
}

#[test]
fn abbreviation_resolution_works() {
    let resolved_abbreviations = resolve_dependent_tasks(&vec![
        Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
        Todo::new(TaskStatus::InProgress, "Specify Format", vec!["B"]),
        Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["SF"]),
        Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["B"]),
        Todo::new(TaskStatus::Waiting, "Command Line", vec!["IP", "DG"])
    ]);

    assert_eq!(resolved_abbreviations["Brainstorm"], vec!["Specify Format", "DOT Generator"]);
    assert_eq!(resolved_abbreviations["Specify Format"], vec!["Implement Parser"]);
    assert_eq!(resolved_abbreviations["Implement Parser"], vec!["Command Line"]);
    assert_eq!(resolved_abbreviations["DOT Generator"], vec!["Command Line"]);
    assert!(resolved_abbreviations["Command Line"].is_empty());
}

#[test]
fn generator_dot_file_works() {
    let generated = generate_dot_file(vec![
        Todo::new(TaskStatus::Completed, "Brainstorm", vec![]),
        Todo::new(TaskStatus::InProgress, "Specify Format", vec!["B"]),
        Todo::new(TaskStatus::Waiting, "Implement Parser", vec!["SF"]),
        Todo::new(TaskStatus::Waiting, "DOT Generator", vec!["B"]),
        Todo::new(TaskStatus::Waiting, "Command Line", vec!["IP", "DG"])
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
> Specify Format [ B ]
- Implement Parser [ SF ]
- DOT Generator [ B ]
- Command Line [ IP, DG ]");

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
