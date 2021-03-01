use std::{collections::HashMap};

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("Our key is {}, and value is {}", key, value);

    let mut database = Database::new().expect("creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    database.flush().expect("flush crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database,std::io::Error> {
        // read the kv.db file
        

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;


        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value");
            // println!("key is {:#?} and value is {:#?}", key, value);
            map.insert(key.to_owned(), value.to_owned());
            
        }
        
        // parse the string
        // populate our map
        Ok(Database {map})
    }

    fn insert(&mut self, key:String, value: String) {
        self.map.insert(key, value);

    }
    fn flush(self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            // let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents)
    }


}

