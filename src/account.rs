use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccount {
    pub account_number: u32, // account number is a 32-bit unsigned integer
    pub holder_name: String,
    pub balance: f64, // balance can be a decimal
}
pub trait Account {
    fn withdraw(&mut self, withdraw_amount: f64);
    fn deposit(&mut self, deposit_amount: f64);
    fn balance(&mut self);
}

impl Account for BankAccount {
    fn deposit(&mut self, deposit_amount: f64) -> () {
        self.balance += deposit_amount;
        println!("Balance after deposit: {}", self.balance);
    }
    fn withdraw(&mut self, withdraw_amount: f64) -> () {
        // check if balance is sufficient
        if withdraw_amount > self.balance {
            println!("Insufficient balance");
            return;
        } else if withdraw_amount < 0.0 {
            println!("Invalid amount");
            return;
        }
        self.balance -= withdraw_amount;
        println!("Balance after withdraw: {}", self.balance);
    }
    fn balance(&mut self) -> () {
        // show balance
        println!("The balance is: {}", self.balance);
    }
}
