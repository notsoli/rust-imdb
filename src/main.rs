mod tsv;
use tsv::{process_titles, process_actors};

mod graph;
use graph::ImdbGraph;

mod input;
use input::select_actor;

static IMDB_DIRECTORY: &str = "imdb/"; // REMEMBER: root is cargo.toml

// static ACTOR_FILE: &str = "name.basics.tsv";
// static TITLE_FILE: &str = "title.basics.tsv";

static ACTOR_FILE: &str = "testActors2.tsv";
static TITLE_FILE: &str = "testMovies2.tsv";

fn main() {
    let mut graph = ImdbGraph::new();
    let title_path = format!("{}{}", IMDB_DIRECTORY, TITLE_FILE);
    let actor_path = format!("{}{}", IMDB_DIRECTORY, ACTOR_FILE);

    // process titles
    println!("Processing titles");
    process_titles(&mut graph, &title_path);
    println!("Ignored {} titles that are not movies",
    graph.diagnostics.invalid_titles);

    // process actors
    println!("Processing actors");
    process_actors(&mut graph, &actor_path);
    println!("Ignored {} people who aren't actors and {} references to invalid titles",
    graph.diagnostics.invalid_people, graph.diagnostics.invalid_actor_refs);

    // print diagnostic information
    println!("Parsed {} movies and {} actors\n",
    &graph.titles.len(), &graph.actors.len());

    for (_, value) in &graph.actors {
        println!("{value}");
    }

    let (source, destination) = select_actor(&mut graph);
    println!("Going from {} to {}", source, destination);
}

