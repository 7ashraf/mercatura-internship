use std::io::{self, Write};
use rusqlite::{params, Connection, Result};

#[derive(Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}


impl Product {
    pub fn new(id: i32, name: String, description: String, price: f64, quantity: u32) -> Product {
        Product {
            id,
            name,
            description,
            price,
            quantity,
        }
    }

    pub fn display(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Price: ${:.2}", self.price);
        println!("Quantity: {}", self.quantity);
    }

    pub fn add_product(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO products (name, description, price, quantity) VALUES (?1, ?2, ?3, ?4)",
            params![self.name, self.description, self.price, self.quantity],
        )?;
        Ok(())
    }

    pub fn edit_product(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM products WHERE id = ?1")?;

  
        let product_exists: bool = stmt.query_row([self.id], |row| {
            let count: i64 = row.get(0)?;
            Ok(count > 0)
        })?;
        
        if !product_exists {
            return Err(rusqlite::Error::InvalidQuery);
        }
        if self.price < 0.0 {
            return Err(rusqlite::Error::InvalidQuery);
        }
        if self.quantity < 0 {
            return Err(rusqlite::Error::InvalidQuery);
        }
        conn.execute(
            "UPDATE products SET name = ?1, description = ?2, price = ?3, quantity = ?4 WHERE id = ?5",
            params![self.name, self.description, self.price, self.quantity, self.id],
        )?;
        Ok(())
    }

    pub fn delete_product(&self, conn: &Connection) -> Result<()> {
        // Check if the product exists
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM products WHERE id = ?1")?;
        let product_exists: bool = stmt.query_row([self.id], |row| {
            let count: i64 = row.get(0)?;
            Ok(count > 0)
        })?;
        if !product_exists {
            return Err(rusqlite::Error::InvalidQuery);
        }
        conn.execute("DELETE FROM products WHERE id = ?1", params![self.id])?;
        Ok(())
    }
    pub fn fetch_all(conn: &Connection) -> Result<Vec<Product>> {
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
        
        let mut products = Vec::new();
        for product in product_iter {
            products.push(product?);
        }
        Ok(products)
    }

    pub fn fetch_by_id(conn: &Connection, id: i32) -> Result<Product> {
        let mut stmt = conn.prepare("SELECT id, name, description, price, quantity FROM products WHERE id = ?1")?;
        let product = stmt.query_row(params![id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                quantity: row.get(4)?,
            })
        })?;
        Ok(product)
    }
}








