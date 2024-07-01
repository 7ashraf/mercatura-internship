use rusqlite::{params, Connection, Result};
use crate::services::inventory::Product; // Import Product from inventory module
use std::io::{self, Write}; // Import io module and Write trait

#[derive(Clone)]

pub struct Purchase {
    pub id: i32,
    pub product_id: i32,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
}

impl Purchase {
    pub fn new(id: i32, product_id: i32, quantity_purchased: u32, purchase_price: f64) -> Purchase {
        Purchase {
            id,
            product_id,
            quantity_purchased,
            purchase_price,
        }
    }

    pub fn add_purchase(&self, conn: &Connection) -> Result<Purchase> {
        conn.execute(
            "INSERT INTO purchases (product_id, quantity_purchased, purchase_price) VALUES (?1, ?2, ?3)",
            params![self.product_id, self.quantity_purchased, self.purchase_price],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Purchase { id: id as i32, ..self.clone() })
    }

    pub fn fetch_all(conn: &Connection) -> Result<Vec<Purchase>> {
        let mut stmt = conn.prepare("SELECT id, product_id, quantity_purchased, purchase_price FROM purchases")?;
        let purchase_iter = stmt.query_map([], |row| {
            Ok(Purchase {
                id: row.get(0)?,
                product_id: row.get(1)?,
                quantity_purchased: row.get(2)?,
                purchase_price: row.get(3)?,
            })
        })?;
        
        let mut purchases = Vec::new();
        for purchase in purchase_iter {
            purchases.push(purchase?);
        }
        Ok(purchases)
    }

    pub fn display(&self, conn: &Connection) -> Result<()> {
        let product = Product::fetch_by_id(conn, self.product_id)?;
        let total_cost = self.quantity_purchased as f64 * self.purchase_price;
        
        println!("Purchase ID: {}", self.id);
        println!("Product Name: {}", product.name);
        println!("Quantity Purchased: {}", self.quantity_purchased);
        println!("Purchase Price: ${:.2}", self.purchase_price);
        println!("Total Cost: ${:.2}", total_cost);
        Ok(())
    }
}

pub fn init_purchases_table(conn: &Connection) -> Result<()> {
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
    Ok(())
}





