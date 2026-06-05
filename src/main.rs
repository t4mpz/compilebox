mod commands {
    pub mod utils;
    pub mod package;
}
mod files {
    pub mod files; 
}


fn main() {
    let content = files::files::read_package_file("test_file.txt").unwrap();
    println!("{:?}", content);
}
