// Modules for lazy package reading implementation
// it writes the file original pkg file and dumps a binary version
// inside the binary version theres a hash from the latest file reading
// the hash checks if the file has been overwritten or not
//

use std::hash::{DefaultHasher, Hash};
use std::file;
use std::io::Read;
use crate::commands::package::Package;



fn hash_string(contents: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = DefaultHasher::new();
    let hashed_contents = contents.clone();
    hashed_contents.hash(&mut hasher);

    Ok(hashed_contents)
}

