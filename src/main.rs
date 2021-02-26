use std::collections::HashMap;

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        // read the kv.db file
        // parse the string
        // populate our map
        Database {map: HashMap::new()}
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let content = format!("{}\t{}\n", key, value);


    std::fs::write("kv.db", content).unwrap();

    let database = Database::new();

}