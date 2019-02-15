use std::collections::HashMap;

use crate::parser::Todo;

pub fn split_identifier(identifier: &str) -> Vec<String> {
    identifier.split(" ").map(|part| part.to_lowercase()).collect()
}

pub fn split_abbreviation(abbreviation: &str) -> Vec<String> {
    let mut parts = Vec::new();

    let remainder = abbreviation.chars()
        .fold(String::new(), |mut current, character| {
            if character.is_uppercase() && current.len() != 0 {
                parts.push(current);
                character.to_string()
            } else {
                current.push(character);
                current
            }
        });

    if !remainder.is_empty() {
        parts.push(remainder);
    }

    parts.iter().map(|part| part.to_lowercase()).collect()
}

pub fn abbreviation_matches(identifier: &str, abbreviation: &str) -> bool {
    let identifier_words = split_identifier(identifier);
    let abbreviation_parts = split_abbreviation(abbreviation);

    if identifier_words.len() != abbreviation_parts.len() {
        return false
    }

    abbreviation_parts
        .iter()
        .zip(identifier_words.iter())
        .all(|(abbreviation_part, identifier_word)| identifier_word.starts_with(abbreviation_part))
}

pub fn resolve_dependent_tasks(tasks: Vec<Todo>) -> Option<HashMap<String, Vec<String>>> {
    let mut dependent_tasks = HashMap::new();

    for task in tasks.iter() {
        dependent_tasks.insert(task.identifier.clone(), Vec::new());
    }

    for task in tasks.iter() {
        for dependency_abbreviation in task.dependencies.clone() {
            match tasks.iter().find(|task| abbreviation_matches(&task.identifier, &dependency_abbreviation)) {
                Some(dependency) => dependent_tasks
                    .entry(dependency.identifier.clone())
                    .and_modify(|dependent_tasks| dependent_tasks.push(task.identifier.clone())),
                None => return None
            };
        }
    }

    Some(dependent_tasks)
}
