use std::str::FromStr;
use std::{
    collections::{hash_map, HashMap},
    io::Read,
    vec,
};

struct Todo {
    //rust in-built hashmap to store key-val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // allocate an empty HashMap
        let mut map = HashMap::new();

        // loop over each lines of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");

            // insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }

        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // we pass true/false as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main() {
    // println!("Hello, world!");
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an action");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed!");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo Saved!"),
            Err(err) => println!("An error has occured: {}", err),
        }
    }
}
