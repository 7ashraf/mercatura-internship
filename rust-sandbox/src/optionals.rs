pub fn run(){
    let name: Option<&str> = Some("Ashraf");
    let age: Option<u8> = Some(22);
    match name {
        Some(name) => {
            match age {
                Some(age) => {
                    println!("My name is {name} and I am {age} years old", name=name, age=age);
                },
                None => {
                    println!("My name is {name} and I am ageless", name=name);
                }
            }
        },
        None => {
            match age {
                Some(age) => {
                    println!("I am nameless and I am {age} years old", age=age);
                },
                None => {
                    println!("I am nameless and I am ageless");
                }
            }
        }
    }
}