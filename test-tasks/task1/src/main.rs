
fn concat_strings(s1: &str, s2:&str) -> String {
    let mut s = String::new();
    s.push_str(s1);
    s.push_str(s2);
    s
}

fn main() {
    let s1 = "Hello, ";
    let s2 = "World!";
    let s = concat_strings(s1, s2);
    println!("{}", s);
    
}
