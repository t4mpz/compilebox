use std::fs::File;
use std::io::Read;
use std::collections::HashMap;



pub fn read_package_file(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut results: HashMap<String, String> = HashMap::new();

    for &line in &lines {
        if line.contains("=") {
            let splitted_options: Vec<&str> = line.split("=").collect();

            let str_values = splitted_options.into_iter().map(|y: &str| String::from(y)).collect::<Vec<String>>();

            results.insert(str_values[0].clone(), str_values[1].clone());
        }
    }

    Ok(results)
}
