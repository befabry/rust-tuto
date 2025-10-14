#[derive(Debug)]
struct Account {
    id: u32,
    balance: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// & is a reference to the variable, and is read-only
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

// &mut is a mutable reference, it is writable / editable
fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account)
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    change_account(&mut account);

    // TODO: call the 'add_account' function here
    add_account(&mut bank, account);

    print!("{:#?}", bank);
}
