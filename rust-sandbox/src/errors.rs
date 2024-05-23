pub fn run(){
    let value :Result<&str, ()> = Ok("Hello");
    match value {
        Ok(value) => {
            println!("Value: {}", value);
        },
        Err(_) => {
            println!("Error");
        }
    }
    let new_value :Result<String, ()> = get_value();
    match new_value {
        Ok(value) => {
            println!("Value: {}", value);
        },
        Err(_) => {
            println!("Error");
        }
    }
    let error :Result<String, ()> = get_error();
    match error {
        Ok(value) => {
            println!("Value: {}", value);
        },
        Err(_) => {
            println!("Error");
        }
    }
}

fn get_value() -> Result<String, ()>{
    Ok("Hello".to_string())
}
fn get_error() -> Result<String, ()>{
    Err(())
}