use std::collections::HashMap;

pub fn run(){
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("name", "Ashraf");
    values.insert("age", "22");
    values.entry("gender").or_insert("m");
    println!("{:?}", values);
}