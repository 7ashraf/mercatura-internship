use std::io::{self, Write};
use rusqlite::{params, Connection, Result};

struct Calculation {
    id: i32,
    operation: String,
    num1: f64,
    num2: f64,
    result: f64,
}
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err(String::from("Error: Division by zero"))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn perform_calculation(conn: &Connection) -> Result<()> {
    let first_number = read_input("Enter the first number: ")
        .and_then(|input| parse_f64(&input))
        .unwrap_or_else(|err| {
            println!("{}", err);
            std::process::exit(1);
        });

    let operation = read_input("Enter the operation (+, -, *, /): ")
        .unwrap_or_else(|err| {
            println!("{}", err);
            std::process::exit(1);
        });

    let second_number = read_input("Enter the second number: ")
        .and_then(|input| parse_f64(&input))
        .unwrap_or_else(|err| {
            println!("{}", err);
            std::process::exit(1);
        });

    let op = match operation.trim() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            std::process::exit(1);
        }
    };

    match calculate(op) {
        Ok(result) => {
            println!("Result: {}", result);
            conn.execute(
                "INSERT INTO calculations (operation, num1, num2, result) VALUES (?1, ?2, ?3, ?4)",
                params![operation.trim(), first_number, second_number, result],
            )?;
        }
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("calculations.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calculations (
            id INTEGER PRIMARY KEY,
            operation TEXT NOT NULL,
            num1 REAL NOT NULL,
            num2 REAL NOT NULL,
            result REAL NOT NULL
        )",
        [],
    )?;

    loop {
        println!("Choose an option:");
        println!("1. Perform a new calculation");
        println!("2. View past results");
        println!("3. Exit");

        let choice = read_input("Enter your choice: ").unwrap_or_else(|err| {
            println!("{}", err);
            std::process::exit(1);
        });

        match choice.trim() {
            "1" => perform_calculation(&conn)?,
            "2" => view_past_results(&conn)?,
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}

fn read_input(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| "Failed to flush stdout".to_string())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    Ok(input.trim().to_string())
}

fn parse_f64(input: &str) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| format!("Failed to parse '{}' as a number", input))
}
fn view_past_results(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, operation, num1, num2, result FROM calculations")?;
    let calculations_iter = stmt.query_map([], |row| {
        Ok(Calculation {
            id: row.get(0)?,
            operation: row.get(1)?,
            num1: row.get(2)?,
            num2: row.get(3)?,
            result: row.get(4)?,
        })
    })?;

    for calc in calculations_iter {
        let calc = calc?;
        println!(
            "ID: {}, Operation: {} {} {} = {}",
            calc.id, calc.num1, calc.operation, calc.num2, calc.result
        );
    }

    Ok(())
}