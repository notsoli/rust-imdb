extern crate csv;
use std::fs::File;
use crate::graph::ImdbGraph;

// opens a specified TSV file and returns a vector of valid TSV entries
fn process_tsv(
    file_path: &str,
) -> Vec<csv::StringRecord> {
    // open file
    let actor_file = File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
    // change delimiter to support tsv
    .delimiter(b'\t')
    .from_reader(actor_file);

    // discard invalid records
    let mut records: Vec<csv::StringRecord> = Vec::new();
    let mut invalid_records: u32 = 0;
    for result in rdr.records() {
        match result {
            Err(_) => { invalid_records += 1; },
            Ok(data) => { records.push(data); }
        }
    }
    println!("Ignored {} invalid records", invalid_records);
    records
}

// populates the graph with movies
pub fn process_titles(graph: &mut ImdbGraph, file_path: &str) {
    let records = process_tsv(file_path);
    for record in records {
        // make sure title is a movie
        if &record[1] == "movie" {
            graph.push_title_vertex(record[0].to_string(), record[2].to_string());
        } else { graph.diagnostics.invalid_titles += 1; }
    }
}

// populates the graph with actors and creates edges between actors and movies
pub fn process_actors(graph: &mut ImdbGraph, file_path: &str) {
    let records = process_tsv(file_path);
    for record in records {
        // get the primary actor role
        let mut primary_role = "";
        let primary_role_result = record[4].split(",").next();
        match primary_role_result {
            None => { println!("Error processing actor roles.") },
            Some(data) => { primary_role = data }
        }

        // make sure person is an actor or actress
        if primary_role == "actor" || primary_role == "actress" {
            // identify involved projects that exist in parsed title data
            for title in record[5].split(",") {
                let result = graph.titles.get(title);
                match result {
                    None => { graph.diagnostics.invalid_actor_refs += 1; },
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
        } else { graph.diagnostics.invalid_people += 1; }
    }
}