#[macro_use]
extern crate nom;

mod parser;
mod abbreviations;
mod generator;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
}
