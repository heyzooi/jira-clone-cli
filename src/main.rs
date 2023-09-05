use std::fs::File;
use std::io::Read;
use std::path::Path;

mod models;

fn main() {
    let mut file = File::open(Path::new("data/db.json")).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();
    let db_state = serde_json::from_str::<models::DBState>(&json).unwrap();
    println!("{:?}", db_state);
}
