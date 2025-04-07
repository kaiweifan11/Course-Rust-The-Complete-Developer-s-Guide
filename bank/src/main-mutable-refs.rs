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

// &mut => mutable reference
fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    let bank = Bank::new();
    let mut account = Account::new(
        1,
        String::from("me")
    );
    // &mut => mutable reference
    change_account(&mut account);
    println!("{:#?}", account);

    // you can male a mutable reference only if there are no immutable ref in use
    // let account_ref = &account; // immutable ref
    // change_account(&mut account); // compile error
    // println!("{:#?}", account_ref.holder);
    
    // only one mutable ref can exist at a time
    // let account_ref_mut = &mut account; // another mutable ref
    // change_account(&mut account); // compile error
    // println!("{:#?}", account_ref_mut.holder);

    // you cant mutate a value through the owner when any ref(mutable or immutable) exists
    // let account_ref_mut = &mut account;
    // account.balance = 100; // compile error, cannot mutate pwner value
    // change_account(account_ref_mut);
    // println!("{:#?}", account);

    // Some types of values are copied instead of moved 
    // numbers, bools, char, array/tuples with copyable elements
    // i.e. breaks ownership
    // let account = Account::new(
    //     1,
    //     String::from("me")
    // ); 
    // let other_account = account;
    // println!("{:#?}", account);  // compile error, values are moved so cannot be accessed, 

    // however this works
    let num = 5;
    let other_num = num;
    println!("{:#?} {:#?}", num, other_num);

    let arr = [5];
    let other_arr = arr;
    println!("{:#?} {:#?}", arr, other_arr);


}
