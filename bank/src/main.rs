#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdrawn(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
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

fn section_3_main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    change_account(&mut account);

    add_account(&mut bank, account);

    println!("{:#?}", bank);
}

fn main() {
    section_3_main();

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdrawn(250);

    println!("{}", account.summary());

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());
}
