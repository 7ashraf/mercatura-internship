mod services {
    pub mod inventory;
    pub mod sales;
    pub mod purchase;
    pub mod reporting;
}

use rusqlite::Connection;
use services::inventory::Product;
use services::sales::Sale;
use services::purchase::Purchase;
use services::reporting::{generate_inventory_report, generate_sales_report, generate_purchase_report};
use std::io::{self, Write};
use rusqlite::{Error, Result};


fn read_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    Ok(input.trim().to_string())
}

fn parse_f64(input: &str) -> Result<f64, rusqlite::Error> {
    input.trim().parse::<f64>()
        .map_err(|err| Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(err)))
}

fn parse_u32(input: &str) -> Result<u32, rusqlite::Error> {
    input.parse::<u32>().map_err(|err| Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(err)))
}
fn parse_i32(input: &str) -> Result<i32, rusqlite::Error> {
    input.parse::<i32>().map_err(|err| Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(err)))
}

fn add_product(conn: &Connection) -> Result<(), rusqlite::Error>{
    let name = read_input("Enter the name of the product: ")?;
    let description = read_input("Enter the description of the product: ")?;
    // let price = read_input("Enter the price of the product: ")
    //     .and_then(|input| parse_f64(&input))
    //     .unwrap_or_else(|err| {
    //         print!("reached");
    //         println!("{}", err);
            
    //         std::process::exit(1);
    //     });

    let price = loop{
        let input = read_input("Enter the price of the product: ")?;
        match parse_f64(&input) {
            Ok(value) => break value,
            Err(err) => println!("{}", err),
        }
    };
    // let quantity = read_input("Enter the quantity of the product: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

        let quantity = loop{
            let input = read_input("Enter the quantity of the product: ")?;
            match parse_u32(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };
    

    let product = Product::new(0, name, description, price, quantity);
    product.add_product(conn)?;
    Ok(())
}



fn edit_product(conn: &Connection) -> rusqlite::Result<()> {
    // let id = read_input("Enter the ID of the product to edit: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     }) as i32;

    let id = loop{
        let input = read_input("Enter the ID of the product to edit: ")?;
        match parse_u32(&input) {
            Ok(value) => break value,
            Err(err) => println!("{}", err),
        }
    } as i32;

    let name = read_input("Enter the new name of the product: ")?;
    let description = read_input("Enter the new description of the product: ")?;
    // let price = read_input("Enter the new price of the product: ")
    //     .and_then(|input| parse_f64(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });
        let price = loop{
            let input = read_input("Enter the new price of the product: ")?;
            match parse_f64(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };
    // let quantity = read_input("Enter the new quantity of the product: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

    let quantity = loop{
        let input = read_input("Enter the quantity of the product: ")?;
        match parse_u32(&input) {
            Ok(value) => break value,
            Err(err) => println!("{}", err),
        }
    };

    let product = Product::new(id, name, description, price, quantity);
    //q what if product does not exist
    //fixed
    match product.edit_product(conn){
        Ok(_) => println!("Product edited successfully"),
        Err(err) => println!("{}", err),
    }
    //product.edit_product(conn)?;
    Ok(())
}

fn delete_product(conn: &Connection) -> rusqlite::Result<()> {
    // let id = read_input("Enter the ID of the product to delete: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     }) as i32;

    
        let id = loop{
            let input = read_input("Enter the ID of the product to edit: ")?;
            match parse_u32(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        } as i32;

    let product = Product::new(id, String::new(), String::new(), 0.0, 0);
    match product.delete_product(conn){
        Ok(_) => println!("Product deleted successfully"),
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn list_products(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, description, price, quantity FROM products")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            price: row.get(3)?,
            quantity: row.get(4)?,
        })
    })?;

    println!("--------------------------------------------------");
    println!("{:<5} {:<20} {:<30} {:<10} {:<10}", "ID", "Name", "Description", "Price", "Quantity");
    println!("--------------------------------------------------");
    for product in product_iter {
        let product = product?;
        println!(
            "{:<5} {:<20} {:<30} {:<10.2} {:<10}",
            product.id, product.name, product.description, product.price, product.quantity
        );
    }
    println!("--------------------------------------------------");

    Ok(())
}

fn inventory_management(conn: &Connection) -> rusqlite::Result<()> {
    loop {
        println!("Inventory Management:");
        println!("1. Add product");
        println!("2. Edit product");
        println!("3. Delete product");
        println!("4. List all products");
        println!("5. Go back");
        println!("6. Terminate");

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => add_product(conn)?,
            "2" => {
                // implement edit_product function and call here
                edit_product(conn)?;
            }
            "3" => {
                // implement delete_product function and call here
                delete_product(conn)?;
            }
            "4" => {
                // implement list_products function and call here
                list_products(conn)?;
            }
            "5" => break,
            "6" => std::process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}

fn record_sale(conn: &Connection) -> Result<(), rusqlite::Error> {
    // let product_id = read_input("Enter the product ID: ")
    //     .and_then(|input| parse_i32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

    let product_id = loop{
        let input = read_input("Enter the product ID: ")?;
        match parse_i32(&input) {
            Ok(value) => break value,
            Err(err) => println!("{}", err),
        }

    };

    // let quantity_sold = read_input("Enter the quantity sold: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

        let quantity_sold = loop {
            let input = read_input("Enter the quantity sold: ")?;
            match parse_u32(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };

    // let sale_price = read_input("Enter the sale price: ")
    //     .and_then(|input| parse_f64(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

        let sale_price = loop {
            let input = read_input("Enter the sale price: ")?;
            match parse_f64(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };

    let sale = Sale::new(0, product_id, quantity_sold, sale_price);
    match sale.add_sale(conn) {
        Ok(val) => println!("Sale recorded successfully"),
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn view_sales(conn: &Connection) -> Result<(), rusqlite::Error> {
    let sales = Sale::fetch_all(conn)?;
    for sale in sales {
        sale.display(conn)?;
        println!("---");
    }
    Ok(())
}


fn sales_management(conn: &Connection) -> rusqlite::Result<()> {
    loop {
        println!("Sales Management:");
        println!("1. Record sale");
        println!("2. View sales");
        println!("3. Go back");
        println!("4. Terminate");

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => {

                record_sale(conn)?;
            }
            "2" => {
                view_sales(conn)?;
            }
            "3" => break,
            "4" => std::process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}

fn record_purchase(conn: &Connection) -> Result<(), rusqlite::Error> {
    // let product_id = read_input("Enter the product ID: ")
    //     .and_then(|input| parse_i32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });
        let product_id = loop {
            let input = read_input("Enter the product ID: ")?;
            match parse_i32(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };

    // let quantity_purchased = read_input("Enter the quantity purchased: ")
    //     .and_then(|input| parse_u32(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

        let quantity_purchased = loop {
            let input = read_input("Enter the quantity purchased: ")?;
            match parse_u32(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };
    

    // let purchase_price = read_input("Enter the purchase price: ")
    //     .and_then(|input| parse_f64(&input))
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         std::process::exit(1);
    //     });

        let purchase_price = loop {
            let input = read_input("Enter the purchase price: ")?;
            match parse_f64(&input) {
                Ok(value) => break value,
                Err(err) => println!("{}", err),
            }
        };

    let purchase = Purchase::new(0, product_id, quantity_purchased, purchase_price);
    match purchase.add_purchase(conn) {
        Ok(val) => println!("Purchase recorded successfully"),
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn view_purchases(conn: &Connection) -> Result<(), rusqlite::Error> {
    let purchases = Purchase::fetch_all(conn)?;
    for purchase in purchases {
        purchase.display(conn)?;
        println!("---");
    }
    Ok(())
}

fn purchase_management(conn: &Connection) -> rusqlite::Result<()> {
    loop {
        println!("Purchase Management:");
        println!("1. Record purchase");
        println!("2. View purchases");
        println!("3. Go back");
        println!("4. Terminate");

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => {
                // implement record_purchase function and call here
                record_purchase(conn)?;
            }
            "2" => {
                // implement view_purchases function and call here
                view_purchases(conn)?;
            }
            "3" => break,
            "4" => std::process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}

fn reporting(conn: &Connection) -> rusqlite::Result<()> {
    loop {
        println!("Reporting:");
        println!("1. Generate inventory report");
        println!("2. Generate sales report");
        println!("3. Generate purchase report");
        println!("4. Go back");
        println!("5. Terminate");

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => generate_inventory_report(conn)?,
            "2" => generate_sales_report(conn)?,
            "3" => generate_purchase_report(conn)?,
            "4" => break,
            "5" => std::process::exit(0),
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open("inventory.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            quantity_sold INTEGER NOT NULL,
            sale_price REAL NOT NULL,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS purchases (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            quantity_purchased INTEGER NOT NULL,
            purchase_price REAL NOT NULL,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )",
        [],
    )?;

    loop {
        println!("Main Menu:");
        println!("1. Inventory Management");
        println!("2. Sales Management");
        println!("3. Purchase Management");
        println!("4. Reporting");
        println!("5. Terminate");

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => inventory_management(&conn)?,
            "2" => sales_management(&conn)?,
            "3" => purchase_management(&conn)?,
            "4" => reporting(&conn)?,
            "5" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}
