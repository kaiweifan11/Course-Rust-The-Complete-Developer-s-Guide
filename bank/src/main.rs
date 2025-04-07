#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0
        }
    }

    fn deposit(&mut self, amount: i32, ) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32, ) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
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

/* LIFETIMES */
// When the fn finishes, values are dropped(removed from memory)
// There will be a compile error if
// you try to return the ref to the value at the end of the fn
// fn make_and_print_account() -> &Account{
//     let account = Account::new(1, String::from("me"));
//     println!("{:#?}", account);

//     &account // compile error
// }

fn main() {
    // make_and_print_account();

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me")); 

    account.deposit(500);
    account.withdraw(250);

    println!("{}", account.summary());

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());
}
