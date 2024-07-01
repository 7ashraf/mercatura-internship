use rusqlite::{params, Connection, Result};
use crate::services::inventory::Product;
use std::io::{self, Write}; 

#[derive(Clone)]

pub struct Sale {
    pub id: i32,
    pub product_id: i32,
    pub quantity_sold: u32,
    pub sale_price: f64,
}

impl Sale {
    pub fn new(id: i32, product_id: i32, quantity_sold: u32, sale_price: f64) -> Sale {
        Sale {
            id,
            product_id,
            quantity_sold,
            sale_price,
        }
    }

    pub fn add_sale(&self, conn: &Connection) -> Result<Sale> {
        conn.execute(
            "INSERT INTO sales (product_id, quantity_sold, sale_price) VALUES (?1, ?2, ?3)",
            params![self.product_id, self.quantity_sold, self.sale_price],
        )?;
        let id = conn.last_insert_rowid();
        Ok(Sale { id: id as i32, ..self.clone() })
    }

    pub fn fetch_all(conn: &Connection) -> Result<Vec<Sale>> {
        let mut stmt = conn.prepare("SELECT id, product_id, quantity_sold, sale_price FROM sales")?;
        let sales_iter = stmt.query_map([], |row| {
            Ok(Sale {
                id: row.get(0)?,
                product_id: row.get(1)?,
                quantity_sold: row.get(2)?,
                sale_price: row.get(3)?,
            })
        })?;
        
        let mut sales = Vec::new();
        for sale in sales_iter {
            sales.push(sale?);
        }
        Ok(sales)
    }

    pub fn display(&self, conn: &Connection) -> Result<()> {
        let product = Product::fetch_by_id(conn, self.product_id)?;
        let total_sales = self.quantity_sold as f64 * self.sale_price;
        let profit = (self.sale_price - product.price) * self.quantity_sold as f64;
        
        println!("Sale ID: {}", self.id);
        println!("Product Name: {}", product.name);
        println!("Quantity Sold: {}", self.quantity_sold);
        println!("Sale Price: ${:.2}", self.sale_price);
        println!("Total Sales: ${:.2}", total_sales);
        println!("Profit: ${:.2}", profit);
        Ok(())
    }
}





















pub fn read_input(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| "Failed to flush stdout".to_string())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    Ok(input.trim().to_string())
}

pub fn parse_f64(input: &str) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| format!("Failed to parse '{}' as a number", input))
}

pub fn parse_u32(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|_| format!("Failed to parse '{}' as a number", input))
}

pub fn parse_i32(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| format!("Failed to parse '{}' as a number", input))
}
