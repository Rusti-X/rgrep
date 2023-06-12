use rgrep;


fn main() {
    let config = rgrep::parse_args()
        .expect("rgrep expected two arguments: 1[to find] 2[file path]");

    let results = rgrep::get_results(&config);

    if results.is_empty() {
        println!("Query '{}' not found", config.query);
    }

    for line in results {
        println!("{} | {}", line.number, line.string)
    }
}
