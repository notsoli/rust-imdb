use std::io;
use crate::graph::ImdbGraph;

pub fn select_actor(graph: &mut ImdbGraph) -> (&String, &String) {
    println!("Choose an actor to travel from:");
    let first_choice = ask_user(graph);

    println!("Choose an actor to travel to:");
    let second_choice = ask_user(graph);

    (first_choice, second_choice)
}

// asks the user to pick an actor
fn ask_user(graph: &mut ImdbGraph) -> &String {
    let mut choice = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut choice) {
        Err(_) => { 
            println!("Error reading user input.");
            return ask_user(graph);
        },
        Ok(_) => {
            choice.pop();
            match graph.match_actors(&choice) {
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

fn choose_match<'a>(graph: &mut ImdbGraph, matches: Vec<&'a String>) -> &'a String {
    println!("Choose a match by typing the corresponding number");
    for (index, key) in matches.iter().enumerate() {
        println!("{} - {}", index, graph.print_edges(*key).unwrap());
    }

    let mut choice = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut choice) {
        Err(_) => { 
            println!("Error reading user input.");
            return choose_match(graph, matches);
        },
        Ok(_) => {
            match choice.parse::<usize>() {
                Err(_) => {
                    println!("Error reading user input.");
                    return choose_match(graph, matches);
                },
                Ok(index) => {
                    return matches.get(index).unwrap();
                }
            }
        }
    };
}