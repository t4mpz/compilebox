use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn read_file_as_struct(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines = contents.split('\n');
    let lines = lines.filter(| line | line.contains("=") || line.chars().next().unwrap_or('#') != '#');
    let values = lines.map(| line: &str | {
        let splitted: Vec<&str> = line.split('=').collect();
        return (String::from(splitted[0]), splitted[1..].join("="));
    });

    let results: HashMap<String, String> = HashMap::from_iter(values);

    Ok(results)
}

pub fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    Ok(contents)
}
