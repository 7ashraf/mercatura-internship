struct BankAccount {
    balance: f64,
    account_number: String, 
    holder_name: String,
    transactions: Vec<Transaction>,
    account_type: AccountType,
}
#[derive(Clone)]
#[derive(Debug)]
struct Transaction {
    amount: f64,
    from: String,
    to: String,
}


trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn transfer(&mut self, amount: f64, recipient: &mut BankAccount) -> Result<(), String>;
    fn balance(&mut self) -> f64;
    fn transaction_history(&mut self) -> Vec<Transaction>;
    fn add_interest(&mut self) -> Result<(), String>;
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err(String::from("Error: Cannot deposit a negative amount"));
        }
        self.balance += amount;
        self.transactions.push(Transaction {
            amount: amount,
            from: String::from("External"),
            to: self.holder_name.clone(),
        });
        Ok(())
    }
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err(String::from("Error: Cannot withdraw a negative amount"));
        }
        if amount > self.balance {
            //println!("Error: Insufficient funds");
            return Err(String::from("Error: Insufficient funds"))
        }
        self.balance -= amount;
        self.transactions.push(Transaction {
            amount: amount,
            from: self.holder_name.clone(),
            to: String::from("External"),
        });
        Ok(())
    }
    fn transfer(&mut self, amount: f64, recipient: &mut BankAccount) -> Result<(), String> {
        if amount < 0.0 {
            println!("Error: Cannot transfer a negative amount");
            return Err(String::from("Error: Cannot transfer a negative amount"));
        }
        if amount > self.balance {
            println!("Error: Insufficient funds");
            //error when removing return keyword?
            return Err(String::from("Error: Insufficient funds"));
        }
        self.balance -= amount;
        recipient.balance += amount;
        self.transactions.push(Transaction {
            amount: amount,
            from: self.holder_name.clone(),
            to: recipient.holder_name.clone(),
        });
        Ok(())
    }
    fn balance(&mut self) -> f64 {
        self.balance
    }
    fn transaction_history(&mut self) -> Vec<Transaction> {
        self.transactions.clone()
    }
    fn add_interest(&mut self) -> Result<(), String> {

        let mut rate = 0.0;
        if self.account_type == AccountType::Checking {
            rate = 0.01;
        }
        else if self.account_type == AccountType::Savings {
            rate = 0.02;
        }
        else {
            println!("Error: Unknown account type");
            return Err(String::from("Error: Unknown account type"));
        }
        
        self.balance += self.balance * rate;
        Ok(())
    }
}
#[derive(PartialEq)]
enum AccountType {
    Savings,
    Checking,
    Unknown,
}
fn main() {
    let mut account1 = BankAccount {
        balance: 0.0,
        account_number: String::from("1234567890"),
        holder_name: String::from("John Doe"),
        transactions: Vec::new(),
        account_type: AccountType::Savings,
    };

    let mut account2 = BankAccount {
        balance: 0.0,
        account_number: String::from("0987654321"),
        holder_name: String::from("Jane Smith"),
        transactions: Vec::new(),
        account_type: AccountType::Checking,
    };

    //q is this the correcet way to call methods that return Result? in order to detect errors?

    match account1.deposit(100.0) {
        Ok(()) => println!("Deposit successful"),
        Err(err) => println!("{}", err),
    }

    match account1.deposit(-1.0) {
        Ok(()) => println!("Deposit successful"),
        Err(err) => println!("{}", err),
    }
        //how to discover errors?

    match account1.withdraw(50.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("{}", err),
    }
    match account1.withdraw(-1.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("{}", err),
    }
    match account2.withdraw(100.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("{}", err),
    }


    println!("Account 1 balance: {}", account1.balance());
    println!("Account 2 balance: {}", account2.balance());

    println!("Account 1 transaction history: {:?}", account1.transaction_history());
    println!("Account 2 transaction history: {:?}", account2.transaction_history());

    account1.add_interest();
    account2.add_interest();

    println!("Account 1 updated balance: {}", account1.balance());
    println!("Account 2 updated balance: {}", account2.balance());
}
