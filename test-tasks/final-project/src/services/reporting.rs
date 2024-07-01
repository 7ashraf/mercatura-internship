
use crate::services::inventory::Product; // Import Product from inventory module
use std::io::{self, Write}; // Import io module and Write trait
use rusqlite::{params, Connection, Result};
use crate::services::sales::Sale;
use crate::services::purchase::Purchase;





pub fn generate_inventory_report(conn: &Connection) -> Result<()> {
    let products = Product::fetch_all(conn)?;
    println!("Inventory Report:");
    println!("--------------------------------------------------");
    println!("{:<5} {:<20} {:<10} {:<10} {:<10}", "ID", "Name", "Price", "Quantity", "Total Value");
    println!("--------------------------------------------------");

    for product in products {
        let total_value = product.price * product.quantity as f64;
        println!("{:<5} {:<20} ${:<10.2} {:<10} ${:<10.2}", product.id, product.name, product.price, product.quantity, total_value);
    }
    println!("--------------------------------------------------");
    Ok(())
}

pub fn generate_sales_report(conn: &Connection) -> Result<()> {
    let sales = Sale::fetch_all(conn)?;
    println!("Sales Report:");
    println!("--------------------------------------------------");
    println!("{:<5} {:<20} {:<10} {:<10} {:<10} {:<10}", "ID", "Product Name", "Qty Sold", "Sale Price", "Total Sales", "Profit");
    println!("--------------------------------------------------");

    for sale in sales {
        sale.display(conn)?;
    }
    println!("--------------------------------------------------");
    Ok(())
}

pub fn generate_purchase_report(conn: &Connection) -> Result<()> {
    let purchases = Purchase::fetch_all(conn)?;
    println!("Purchase Report:");
    println!("--------------------------------------------------");
    println!("{:<5} {:<20} {:<10} {:<10} {:<10}", "ID", "Product Name", "Qty Purchased", "Purchase Price", "Total Cost");
    println!("--------------------------------------------------");

    for purchase in purchases {
        purchase.display(conn)?;
    }
    println!("--------------------------------------------------");
    Ok(())
}
