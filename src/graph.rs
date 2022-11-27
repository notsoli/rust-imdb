use std::collections::HashMap;
use std::collections::HashSet;

pub struct Diagnostics {
    pub invalid_titles: u32,
    pub invalid_title_records: u32,
    pub invalid_people: u32,
    pub invalid_actor_records: u32,
    pub invalid_actor_refs: u32,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics {
            invalid_titles: 0,
            invalid_title_records: 0,
            invalid_people: 0,
            invalid_actor_records: 0,
            invalid_actor_refs: 0,
        }
    }
}

pub struct ImdbGraph {
    pub titles: HashMap<String, String>,
    pub actors: HashMap<String, String>,
    title_edges: HashMap<String, Vec<String>>,
    actor_edges: HashMap<String, Vec<String>>,
    pub diagnostics: Diagnostics,
}

impl ImdbGraph {
    pub fn new() -> ImdbGraph {
        ImdbGraph {
            titles: HashMap::new(),
            actors: HashMap::new(),
            title_edges: HashMap::new(),
            actor_edges: HashMap::new(),
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn push_title_vertex(self: &mut ImdbGraph, from: String, to: String) {
        self.titles.insert(from, to);
    }

    pub fn push_actor_vertex(self: &mut ImdbGraph, from: String, to: String) {
        self.actors.insert(from, to);
    }

    pub fn push_title_edge(self: &mut ImdbGraph, from: String, to: String) {
        self.title_edges.entry(from).or_default().push(to);
    }

    pub fn push_actor_edge(self: &mut ImdbGraph, from: String, to: String) {
        self.actor_edges.entry(from).or_default().push(to);
    }

    pub fn print_edges(self: &ImdbGraph, key: &String) -> Option<String> {
        match self.actor_edges.get(key) {
            None => {
                return None;
            }
            Some(keys) => {
                let mut title_names: Vec<String> = Vec::new();
                for key in keys {
                    title_names.push(self.titles.get(key).unwrap().clone());
                }
                return Some(title_names.join(", "));
            }
        }
    }

    pub fn match_actors(self: &ImdbGraph, keyword: &str) -> Option<Vec<String>> {
        let mut matches: Vec<String> = Vec::new();
        for (key, value) in &self.actors {
            if *value == keyword {
                matches.push(key.clone())
            }
        }
        match matches.len() {
            0 => None,
            _ => Some(matches),
        }
    }

    pub fn traverse<'a>(
        self: &'a ImdbGraph,
        source: &'a String,
        destination: &String,
    ) -> Option<HashSet<&String>> {
        // create a list of future and visited nodes
        let mut queue: Vec<(&String, bool)> = Vec::new();
        queue.push((source, true));
        let mut visited: HashSet<&String> = HashSet::new();

        // iterate through queue until it's empty or a match has been found
        let mut found = false;
        while !queue.is_empty() && !found {
            // bump the queue
            let (current_ref, is_actor) = queue.pop().unwrap();

            // make sure current node hasn't been visited already
            if visited.get(current_ref) == None {
                // add to visited
                visited.insert(current_ref);

                // determine what hashmap to draw from
                let target_list = if is_actor {
                    &self.actor_edges } else { &self.title_edges };

                // add neighbors to queue
                match target_list.get(current_ref) {
                    Some(edges) => {
                        for edge in edges {
                            if edge == destination {
                                found = true;
                                break;
                            } else {
                                queue.push((edge, !is_actor));
                            }
                        }
                    }
                    None => {}
                }
            }
        }

        if found {
            return Some(visited);
        } else {
            return None;
        }
    }

    pub fn generate_path(self: &ImdbGraph, keys: HashSet<&String>, source: &String, destination: &String) -> String {
        let mut names: Vec<String> = Vec::new();
        let mut current_node = source;
        let mut is_actor = true;
        while current_node != destination {
            let target_list = if is_actor {
                &self.actors } else { &self.titles };
            names.push(target_list.get(current_node).unwrap().clone());

            current_node = keys.get(current_node).unwrap();
            is_actor = !is_actor;
        }

        names.join(", ")
    }
}
