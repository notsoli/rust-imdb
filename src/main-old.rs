extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;
use std::fmt;
use std::collections::HashMap;

static IMDB_DIRECTORY: &str = "../imdb/";
static ACTOR_FILE: &str = "someActors.tsv";
static TITLE_FILE: &str = "someMovies.tsv";

struct Node {
    title: String,
    neighbors: Vec<Node>
}

type NodeResult = Result<(String, Node), ProcessTSVError>;
type NodeVecResult = Result<HashMap<String, Node>, Box<dyn Error>>;

fn main() {
    let title_path: String = format!("{}{}", IMDB_DIRECTORY, TITLE_FILE);
    if let Err(err) = process_tsv(&title_path, process_title) {
        println!("{}", err);
        process::exit(1);
    }
    let actor_path: String = format!("{}{}", IMDB_DIRECTORY, ACTOR_FILE);
    if let Err(err) = process_tsv(&actor_path, process_actor) {
        println!("{}", err);
        process::exit(1);
    }
}

// opens a specified TSV file and converts each entry into a node using a specified function
fn process_tsv(
    file_path: &str,
    process_record: fn(csv::StringRecord) -> NodeResult
) -> NodeVecResult {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let actor_file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(actor_file);
    for result in rdr.records() {
        let record = result?;
        let (id, node) = process_record(record)?;
        nodes.insert(id, node);
    }
    Ok(nodes)
}

// allows error propagation for processing TSV entriesx
#[derive(Debug)]
struct ProcessTSVError {
    details: String
}

impl ProcessTSVError {
    fn new(msg: &str) -> ProcessTSVError {
        ProcessTSVError{details: msg.to_string()}
    }
}

impl fmt::Display for ProcessTSVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ProcessTSVError {
    fn description(&self) -> &str {
        &self.details
    }
}

// converts an IMDB title entry into a node object
fn process_title(record: csv::StringRecord) -> NodeResult {
    if &record[1] != "movie" {
        return Err(ProcessTSVError::new("Title is not a movie."));
    }
    Ok((record[0].to_string(), Node {
        title: record[2].to_string(),
        neighbors: Vec::new()
    }))
}

// converts an IMDB actor entry into a node object
fn process_actor(record: csv::StringRecord) -> NodeResult {
    // make sure the person is an actor
    let primary_role: &str;
    let primary_role_result = record[4].split(",").next();
    match primary_role_result {
        None => { return Err(ProcessTSVError::new("Error processing actor roles.")) },
        Some(data) => { primary_role = data }
    }

    Ok((record[0].to_string(), Node {
        title: String::from("testName"),
        neighbors: Vec::new()
    }))
}

