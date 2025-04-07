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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_holder(holder: String) {
    println!("{:#?}", holder);
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn print_account2(account: Account) -> Account {
    println!("{:#?}", account);
    /* moving values back to it original ownership */
    // return the account ownership
    account
}

/* BORROWING */
// & pass the reference of the account
fn print_account3(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(
        1,
        String::from("me")
    );
    //bank.accounts.push(account);

    /* OWNERSHIP */
    // when reassigned, values inside bank is moved to other_bank
    // so when called println, nothing to print because all values from bank
    // are already moved to other_bank
    // let other_bank = bank; // other_bank now owns the values
    // println!("{:#?}", bank);

    // passing a variable into a function also moves the values
    // print_account(account); // values from account passed to print_account, print_account now owns the values
    // print_account(account); // account no longer has values, cannot compile

    // using a variable moves the values
    // let list_of_accounts = vec![account]; // list_of_accounts now owns values
    // println!("{:#?}", account); // account no longer has values, cannot compile
    
    // values can also be partially moved
    // let accounts = bank.accounts; // bank.accounts moved to accounts
    // println!("{:#?}", bank); // cannot compile because values in pbject partially moved.

    // also cannot access particular value inside variable when it's moved
    // print_account(account);
    // print_account(account.holder);

    // again since the value inside account is moved, account doesnt have value for holder
    // print_holder(account.holder); // account is partially moved(account.holder)
    // print_account(account); // cannot work 
    // println!("{:#?}", account.id); can work if we dont use print_account before this.xs


    /* SOLUTION1: moving values back to it original ownership */
    // add mut 
    // let mut account = Account::new(
    //     2,
    //     String::from("someone else")
    // );
    // print_account2(account);
    // print_account2(account);

    // this also works, passing back values to bank2
    // let mut bank = Bank::new();
    // let other_bank = bank;
    // bank = other_bank;
    // println!("{:#?}", bank);

    /* SOLUTION2: BORROWING */
    // & create a reference to the owner(account)
    let account_ref = &account;
    print_account3(account_ref);

    // mostly we just put the & before passing the param
    print_account3(&account);
    println!("{:#?}", account);

    // you can create many read-only(immutable) references to a value
    let mut account = account;
    account.balance = 10;
    // let account_ref = &account;
    // account_ref.balance = 11; // cannot compile, & refs are immutable

    // these refs can all exist at the same time
    // because & refs are read-only, thats why this is possible. 
    // if multiple refs can be updated at the same time, it will cause problems
    let account = Account::new(
        1,
        String::from("me")
    );
    let account_ref1 = &account;
    let account_ref2 = &account;
    print_account3(account_ref1);
    print_account3(account_ref2);

    // you cannot move a value while a ref to the value exists
    // let other_account = account; // cannot compile because it's moved when borrowed
    // print_account3(account_ref1); // borrowed 

}
