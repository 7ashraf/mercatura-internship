# Product Inventory Management System

This is a simple command-line Product Inventory Management System written in Rust. The system allows users to manage inventory, sales, purchases, and generate reports. The data is stored in a SQLite database.

## Features

- **Inventory Management**
  - Add a product
  - Edit a product
  - Delete a product
  - View all products

- **Sales Management** (Password Protected)
  - Record sales transactions
  - Calculate and display the total sales and profit for each transaction

- **Purchase Management**
  - Record purchase transactions
  - Calculate and display the total cost of each purchase

- **Reporting** (Password Protected)
  - Generate reports showing the store's inventory, sales, and purchase history

## Security

Sales Management and Reporting functionalities are password protected. The default password is `securepassword`.

## Project Structure

The project is structured with each functionality separated into its own service for simplicity and separation of concerns:

- **Main Runner**: Handles initialization and user interface
- **Inventory Service**: Manages product inventory
- **Sales Service**: Manages sales transactions
- **Purchase Service**: Manages purchase transactions
- **Reporting Service**: Generates various reports

## Usage

### Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

### Setup

1. Clone the repository:
    ```bash
    git clone https://github.com/7ashraf/mercatura-internship/tree/main/test-tasks/final-project
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Run the application:
    ```bash
    cargo run
    ```

### Commands

When you run the application, you will be prompted with a menu to choose from the following options:

1. Inventory Management
2. Sales Management (Password Protected)
3. Purchase Management
4. Reporting (Password Protected)
5. Exit

Each menu option will further guide you through the available functionalities.

## Database

The application uses SQLite to store data. The database file `inventory.db` will be created in the root directory of the project.

### Schema

- **Products Table**
    ```sql
    CREATE TABLE products (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        price REAL NOT NULL,
        quantity INTEGER NOT NULL
    );
    ```

- **Sales Table**
    ```sql
    CREATE TABLE sales (
        id INTEGER PRIMARY KEY,
        product_id INTEGER NOT NULL,
        quantity_sold INTEGER NOT NULL,
        sale_price REAL NOT NULL,
        FOREIGN KEY (product_id) REFERENCES products (id)
    );
    ```

- **Purchases Table**
    ```sql
    CREATE TABLE purchases (
        id INTEGER PRIMARY KEY,
        product_id INTEGER NOT NULL,
        quantity_purchased INTEGER NOT NULL,
        purchase_price REAL NOT NULL,
        FOREIGN KEY (product_id) REFERENCES products (id)
    );
    ```

