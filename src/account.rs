use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct BankAccount {
    pub account_number: u32, // account number is a 32-bit unsigned integer
    pub holder_name: String,
    pub balance: f64, // balance can be a decimal
}
pub trait Account {
    fn withdraw(&mut self, withdraw_amount: f64) -> Result<(), String>;
    fn deposit(&mut self, deposit_amount: f64) -> Result<(), String>;
    fn balance(&mut self);
}

impl Account for BankAccount {
    fn deposit(&mut self, deposit_amount: f64) -> Result<(), String> {
        if deposit_amount < 0.0 || deposit_amount.is_nan() {
            return Err("Invalid amount".to_string());
        }
        self.balance += deposit_amount;
        println!("Balance after deposit: {}", self.balance);
        Ok(())
    }
    fn withdraw(&mut self, withdraw_amount: f64) -> Result<(), String> {
        // check if balance is sufficient
        if withdraw_amount > self.balance {
            return Err("Insufficient balance".to_string());
        } else if withdraw_amount < 0.0 {
            return Err("Invalid amount".to_string());
        }
        self.balance -= withdraw_amount;
        println!("Balance after withdraw: {}", self.balance);
        Ok(())
    }
    fn balance(&mut self) -> () {
        // show balance
        println!("The balance is: {}", self.balance);
    }
}
