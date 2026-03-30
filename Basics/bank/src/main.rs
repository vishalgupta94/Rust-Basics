#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
    holder: String
}

impl Account{
    fn new(id: u32, holder: String) -> Self{
        Account{
            id, 
            holder,
            balance: 0
        }
    }
}

#[derive(Debug)]
struct Bank{
    account: Vec<Account>,
}

impl Bank{
    fn new() -> Self {
        Bank{
          account: vec![]
        }
    }
}

fn main() {
    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("Vishal1"));
    let account2 = Account::new(2, String::from("Vishal2"));
    let account3 = Account::new(3, String::from("Vishal3"));
    
    bank.account.push(account1);
    bank.account.push(account2);
    bank.account.push(account3);
    println!("Hello, world! {:#?}", bank);
}
