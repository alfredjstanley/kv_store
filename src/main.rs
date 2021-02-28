use std::{collections::HashMap};

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut database = Database::new().expect("creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database,std::io::Error> {
        // read the kv.db file
        let contents = std::fs::read_to_string("kv.db")?;

        let mut map = HashMap::new();


        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            println!("key is {:#?} and value is {:#?}", key, value);
            map.insert(key.to_owned(), value.to_owned());
            
        }
        
        // parse the string
        // populate our map
        Ok(Database {map: map})
    }

    fn insert(&mut self, key:String, value: String) {
        self.map.insert(key, value);

    }
    fn flush(self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for pairs in self.map {
            let kvpair = format!("{}\t{}", pairs.0, pairs.1);
            contents.push_str(&kvpair);
        }

        std::fs::write("kv.db", contents)
    }


}