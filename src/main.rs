use rand::Rng; // import the Rng trait from the rand crate to generate random numbers
mod account;
mod save_data; // declares the module
use account::{Account, BankAccount};
use save_data::add_account; // brings function into scope // brings struct into scope
fn main() {
    // one way to do it
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: "Alice".to_string(),
        balance: 100.0,
    };
    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: "Bob".to_string(),
        balance: 50.0,
    };

    match account1.deposit(50.0) {
        Ok(_) => println!("Deposit successful"),
        Err(e) => println!("Error: {}", e),
    }
    match account2.withdraw(30.0) {
        Ok(_) => println!("Withdraw successful"),
        Err(e) => println!("Error: {}", e),
    }
    account1.balance();
    account2.balance();
    println!("Thank you for using our banking system!");

    // another way to do it
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
    let mut choice: String = String::new();
    println!("Do you want to deposit, withdraw or see current balance? ");
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    let mut amount = String::new();

    match choice {
        "deposit" => {
            println!("Enter the amount: ");
            std::io::stdin().read_line(&mut amount).unwrap();
            // check if the user enters a valid amount
            let parsed_amount = amount.trim().parse::<f64>().unwrap();
            account.deposit(parsed_amount);
        }
        "withdraw" => {
            println!("Enter the amount: ");
            std::io::stdin().read_line(&mut amount).unwrap();
            let parsed_amount = amount.trim().parse::<f64>().unwrap();
            account.withdraw(parsed_amount);
        }
        "balance" => account.balance(),
        _ => println!("Invalid choice"),
    };
}
