use rand::Rng; // import the Rng trait from the rand crate to generate random numbers
mod account;
mod save_data; // declares the module
use crate::account::{Account, BankAccount};
// use account::{Account, BankAccount}; // brings the BankAccount struct and Account trait into scope
use save_data::add_account; // brings function into scope

fn main() {
    // Generates a random number between 1 and 100
    let mut rng = rand::thread_rng();
    let account_number: u32 = rng.gen_range(1..101);

    // prompt the user to a holder name;
    let mut holder_name: String = String::new();
    println!("if want to open a bank account please enter your name: ");
    std::io::stdin().read_line(&mut holder_name).unwrap();
    let mut account = BankAccount {
        account_number,
        holder_name,
        balance: 0.0,
    };

    println!("hello {} ", account.holder_name);
    print!("Account number: {} ", account.account_number);
    add_account({
        BankAccount {
            account_number,
            holder_name: account.holder_name.clone(),
            balance: 0.0,
        }
    });

    // ask if user wants to deposit or withdraw
    let mut choice = String::new();
    println!("Do you want to deposit, withdraw or see current balance? ");
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    let mut amount = String::new();

    match choice {
        "deposit" => {
            println!("Enter the amount: ");
            std::io::stdin().read_line(&mut amount).unwrap();
            // check if the user enters a valid amount
            match amount.trim().parse::<f64>() {
                Ok(amount) => account.deposit(amount),
                Err(_) => println!("Invalid amount entered"),
            }
        }
        "withdraw" => {
            println!("Enter the amount: ");
            std::io::stdin().read_line(&mut amount).unwrap();
            match amount.trim().parse::<f64>() {
                Ok(amount) => account.withdraw(amount),
                Err(_) => println!("Invalid amount entered"),
            }
        }
        "balance" => account.balance(),
        _ => println!("Invalid choice"),
    }
}
