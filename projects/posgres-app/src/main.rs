use postgres::{Client, NoTls, Error};


fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres password=0000", NoTls)?;

    client.batch_execute("
        CREATE TABLE customer (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            phone    INT NOT NULL
        )
    ")?;
    client.batch_execute("
        CREATE TABLE product (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            price    INT NOT NULL,
            customer_id INT NOT NULL REFERENCES customer
        )
    ")?;
    println!("Hello, world!");  
     
    Ok(())

}
