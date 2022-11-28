use std::collections::HashMap;

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
    ) -> Option<HashMap<&String, &String>> {
        // create a list of future and visited nodes
        let mut queue = vec![(source, source, true)];
        let mut visited: HashMap<&String, &String> = HashMap::new();

        // iterate through queue
        let mut found = false;
        while !queue.is_empty() && !found {
            // bump the queue and add to visited nodes
            let (current, source, is_actor) = queue.pop().unwrap();
            visited.insert(current, source);

            // determine what hashmap to draw from
            let target_list = if is_actor {
                &self.actor_edges } else { &self.title_edges };

            // add neighbors to queue
            match target_list.get(current) {
                Some(edges) => {
                    for edge in edges {
                        // check if the edge is the right node
                        if edge == destination {
                            found = true;
                            visited.insert(edge, current);
                            break;
                        // make sure edge hasn't been visited & isn't already in the queue
                        } else if visited.get(edge) == None 
                        && queue.iter().all(|(i, _, _)| i != &edge) {
                            queue.push((edge, current, !is_actor));
                        }
                    }
                }
                None => {}
            }
        }

        // return visited nodes so a trail can be recreated
        if found { Some(visited) } else { None }
    }

    pub fn generate_path(
        self: &ImdbGraph, 
        keys: HashMap<&String, &String>,
        source: &String,
        destination: &String
    ) -> String {
        // track our progress through the graph
        let mut names: Vec<String> = Vec::new();

        // store the current node and wheter it is an actor
        let mut current = destination;
        let mut is_actor = true;
        while current != source {
            // store the name(!) of the current node
            let target_list = if is_actor {
                &self.actors } else { &self.titles };
            names.push(target_list.get(current).unwrap().clone());

            // crawl back one level
            current = keys.get(current).unwrap();
            is_actor = !is_actor;
        }

        // push the source (since we know we reached it) and make it readable
        names.push(self.actors.get(source).unwrap().clone());
        names.reverse();
        names.join(", ")
    }
}
