use crate::account::BankAccount;
use serde_json;
use std::fs;

type Accounts = Vec<BankAccount>; // A type alias for a list of accounts

fn load_accounts() -> Accounts {
    let data = fs::read_to_string("accounts.json").unwrap_or("[]".to_string()); // Read file, default to empty array
    serde_json::from_str(&data).unwrap_or_else(|_| vec![]) // Convert JSON back to Vec<Account>
}
fn save_accounts(accounts: &Accounts) {
    let serialized = serde_json::to_string_pretty(accounts).unwrap(); // Convert Vec<Account> to JSON
    fs::write("accounts.json", serialized).unwrap(); // Save to file
}
pub fn add_account(new_account: BankAccount) {
    let mut accounts = load_accounts(); // Load existing accounts
    accounts.push(new_account); // Add new account
    save_accounts(&accounts); // Save updated list
}
