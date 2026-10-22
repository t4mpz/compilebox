
pub fn bool_to_string(b: bool) -> String{
    match b {
        true => String::from("true"),
        false => String::from("false")
    }
}

pub fn str_to_bool(b: String) -> bool {
    match b.as_str() {
        "true" => true,
        _ => false
    }
}
