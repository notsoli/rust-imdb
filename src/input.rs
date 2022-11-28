use std::io;
use crate::graph::ImdbGraph;

pub fn select_actor(graph: &ImdbGraph) -> (String, String) {
    println!("Choose an actor to travel from:");
    let first_choice = ask_user(graph);

    println!("Choose an actor to travel to:");
    let second_choice = ask_user(graph);

    (first_choice, second_choice)
}

// asks the user to pick an actor
fn ask_user(graph: &ImdbGraph) -> String {
    let mut choice = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut choice) {
        Err(_) => { 
            println!("Error reading user input.");
            return ask_user(graph);
        },
        Ok(_) => {
            let matches = graph.match_actors(&choice.trim());
            match matches {
                None => {
                    println!("No matches, choose again:");
                    return ask_user(graph);
                },
                Some(mut matches) => {
                    match matches.len() {
                        1 => return matches.pop().unwrap(),
                        _ => return choose_match(graph, matches)
                    }
                }
            }
        }
    }
}

fn choose_match(graph: &ImdbGraph, matches: Vec<String>) -> String {
    println!("Choose a match by typing the corresponding number");
    let mut index = 1;
    let mut valid_matches: Vec<&String> = Vec::new();
    for key in &matches {
        let movies = graph.print_edges(key);
        match movies {
          None => {},
          Some(data) => {
            valid_matches.push(key);
            println!("{} - {}", index, data);
            index += 1;
          }
        }

    }

    let mut choice = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut choice) {
        Err(_) => { 
            println!("Error reading user input.");
            return choose_match(graph, matches);
        },
        Ok(_) => {
            match choice.trim().parse::<usize>() {
                Err(_) => {
                    println!("Error reading user input.");
                    return choose_match(graph, matches);
                },
                Ok(index) => {
                    let key = *valid_matches.get(index-1).unwrap();
                    return key.clone();
                }
            }
        }
    };
}