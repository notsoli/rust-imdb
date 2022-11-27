extern crate csv;
use crate::graph::ImdbGraph;
use std::fs::File;

// opens a specified TSV file and returns a vector of valid TSV entries
fn process_tsv(file_path: &str) -> csv::Reader<File> {
    // open file
    let actor_file = File::open(file_path).unwrap();
    let rdr = csv::ReaderBuilder::new()
        // change delimiter to support tsv
        .delimiter(b'\t')
        .from_reader(actor_file);

    // return reader
    rdr
}

// populates the graph with movies
pub fn process_titles(graph: &mut ImdbGraph, file_path: &str) {
    let mut reader = process_tsv(file_path);
    for record_result in reader.records() {
        match record_result {
            Err(_) => {
                graph.diagnostics.invalid_title_records += 1;
            }
            Ok(record) => {
                // make sure title is a movie
                if &record[1] == "movie" {
                    graph.push_title_vertex(record[0].to_string(), record[2].to_string());
                } else {
                    graph.diagnostics.invalid_titles += 1;
                }
            }
        }
    }
}

// populates the graph with actors and creates edges between actors and movies
pub fn process_actors(graph: &mut ImdbGraph, file_path: &str) {
    let mut reader = process_tsv(file_path);
    for record_result in reader.records() {
        match record_result {
            Err(_) => {
                graph.diagnostics.invalid_actor_records += 1;
            }
            Ok(record) => {
                // get the primary actor role
                let mut primary_role = "";
                let primary_role_result = record[4].split(",").next();
                match primary_role_result {
                    None => {
                        println!("Error processing actor roles.")
                    }
                    Some(data) => primary_role = data,
                }

                // make sure person is an actor or actress
                if primary_role == "actor" || primary_role == "actress" {
                    // identify involved projects that exist in parsed title data
                    for title in record[5].split(",") {
                        let result = graph.titles.get(title);
                        match result {
                            None => {
                                graph.diagnostics.invalid_actor_refs += 1;
                            }
                            Some(_) => {
                                let title = title.to_string();
                                let actor = record[0].to_string();

                                // make a two-way edge between the title and actor
                                graph.push_title_edge(title.clone(), actor.clone());
                                graph.push_actor_edge(actor.clone(), title.clone());
                            }
                        }
                    }

                    // push actor to graph
                    graph.push_actor_vertex(record[0].to_string(), record[1].to_string());
                } else {
                    graph.diagnostics.invalid_people += 1;
                }
            }
        }
    }
}
