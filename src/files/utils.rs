
pub fn bool_to_string(b: bool) -> String{
    match b {
        true => String::from("true"),
        false => String::from("false")
    }
}
