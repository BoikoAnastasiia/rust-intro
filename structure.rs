fn main() {
    let mut account = BankAccount {
        balance: 10000000000.29,
        owner: String::from("Nastya"),
    };
    account.deposit(1500000.0);
    account.check_balance();
}

struct BankAccount {
    balance: f64,
    owner: String,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("New balance of {} is {}", self.owner, self.balance);
    }

    fn check_balance(&self) {
        println!("Balance of {} is {}", self.owner, self.balance);
    }
}