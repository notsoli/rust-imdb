use std::collections::HashMap;

pub struct Diagnostics {
    pub invalid_titles: u32,
    pub invalid_people: u32,
    pub invalid_actor_refs: u32,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics {
            invalid_titles: 0,
            invalid_people: 0,
            invalid_actor_refs: 0
        }
    }
}

pub struct ImdbGraph {
    pub titles: HashMap<String, String>,
    pub actors: HashMap<String, String>,
    title_edges: HashMap<String, Vec<String>>,
    actor_edges: HashMap<String, Vec<String>>,
    pub diagnostics: Diagnostics
}

impl ImdbGraph {
    pub fn new() -> ImdbGraph {
        ImdbGraph {
            titles: HashMap::new(),
            actors: HashMap::new(),
            title_edges: HashMap::new(),
            actor_edges: HashMap::new(),
            diagnostics: Diagnostics::new()
        }
    }
    
    pub fn push_title_vertex(self: &mut ImdbGraph, from: String, to: String) {
        self.titles.insert(from, to);
    }

    pub fn push_actor_vertex(self: &mut ImdbGraph, from: String, to: String) {
        self.actors.insert(from, to);
    }

    pub fn push_title_edge(self: &mut ImdbGraph, from: String, to: String) {
        self.title_edges.entry(from)
        .or_default()
        .push(to);
    }

    pub fn push_actor_edge(self: &mut ImdbGraph, from: String, to: String) {
        self.actor_edges.entry(from)
        .or_default()
        .push(to);
    }

    pub fn print_edges(self: &mut ImdbGraph, key: &String) -> Option<String> {
        match self.actor_edges.get(key) {
            None => { return None; },
            Some(keys) => {
                let mut title_names: Vec<String> = Vec::new();
                for key in keys {
                    title_names.push(self.titles.get(key).unwrap().clone());
                }
                return Some(title_names.join(", "));
            }
        }
    }

    pub fn match_actors(self: &mut ImdbGraph, keyword: &String) -> Option<Vec<&String>> {
        let mut matches: Vec<&String> = Vec::new();
        for (key, value) in &self.actors {
            if value.contains(keyword) { matches.push(key); }
        }
        match matches.len() {
            0 => None,
            _ => Some(matches)
        }
    }
}