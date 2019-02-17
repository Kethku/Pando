#[macro_use]
extern crate nom;
#[macro_use]
extern crate clap;

mod parser;
mod abbreviations;
mod generator;

use std::process::Command;
use std::path::PathBuf;
use std::fs::{File, canonicalize};
use std::env;
use std::io::{Read, Write};
use std::panic;

use clap::App;
use parser::parse_pando;
use generator::generate_dot_file;

pub fn compile(pando_code: &str) -> String {
    generate_dot_file(parse_pando(pando_code))
}

fn resolve_path(relative_path: &str) -> PathBuf {
    let mut absolute_path = std::env::current_dir().expect("Could not resolve current directory");
    absolute_path.push(relative_path);
    absolute_path
}

fn setup_better_panic_messages() {
    panic::set_hook(Box::new(|info| {
        match info.payload().downcast_ref::<String>() {
            Some(text) => println!("{}", text),
            None => println!("Unknown error")
        }
    }));
}

#[cfg(test)]
mod tests;

fn main() {
    setup_better_panic_messages();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let todo_path = canonicalize(matches.value_of("FILE").expect("Need a todo file path")).expect("Todo file does not exist");
    let dot_path = matches.value_of("dot").map(resolve_path).unwrap_or_else(|| {
        let mut temp_path = env::temp_dir();
        temp_path.push(todo_path.with_extension("dot").file_name().expect("Could not get temp path file name"));
        temp_path
    });

    let output_path = matches.value_of("output").map(resolve_path).unwrap_or_else(|| {
        todo_path.with_extension("svg")
    });

    let mut todo_file = File::open(&todo_path).expect("Could not open todo file");
    let mut todo_text = String::new();
    todo_file.read_to_string(&mut todo_text).expect("Could not read todo file");

    let dot_text = compile(&todo_text);
    let mut dot_file = File::create(&dot_path).expect("Could not create dot file");
    dot_file.write_all(dot_text.as_bytes()).expect("Could not write dot file");

    let render_output = Command::new("dot")
        .arg(dot_path)
        .arg("-Tsvg")
        .output()
        .expect("Could not execute graphviz command");

    let rendered_text = String::from_utf8(render_output.stdout).expect("Invalid graphviz output");

    if rendered_text.starts_with("Error") {
        println!("Graphviz error: {}", rendered_text);
    } else {
        let mut output_file = File::create(&output_path).expect("Could not create output file");
        output_file.write_all(rendered_text.as_bytes()).expect("Could not write output file");
        println!("Successfully output to {:?}", output_path);
    }
}
