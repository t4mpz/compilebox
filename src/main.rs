mod commands {
    pub mod utils;
    pub mod package;
    pub mod sys;
}
mod files {
    pub mod files; 
    pub mod lazy;
    pub mod utils;
}

use std::hash::{Hash, Hasher, DefaultHasher};

#[derive(Hash)]
struct Content {
    key: String
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}


fn main() {
    let path = "test_file.txt";
    let str_content = files::files::read_file(path).unwrap();
    let content = files::files::read_file_as_struct(path).unwrap();


    println!("{:?}", content);
    println!("{:?}", str_content);

    let mut scontent = Content{key: "dafuck".to_string()};

    scontent.key = match content.get("Key") {
        Some(k) => k.to_string(),
        None => panic!("No Key value for the struct fudge heck fuck"),
    };

    println!("Hashes");

    println!("Struct: {:?}", calculate_hash(&scontent));
    println!("Str: {:?}", calculate_hash(&str_content));
}
