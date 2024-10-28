struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u32
}
trait Account{
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn balance(&self) -> u32;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u32){
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u32){
        let balance: i32 = self.balance as i32 - amount as i32;
        if balance >= 0 {
            self.balance = balance.try_into().unwrap();
        } else {
            println!("El monto a retirar es mayor al saldo");
        }
        
    }
    fn balance(&self) -> u32 {
        self.balance
    }

}



fn main() {
    let mut bank_account1 = BankAccount {
        account_number: 12345,
        holder_name: String::from("Orlando"),
        balance: 0
    };

    let mut bank_account2 = BankAccount {
        account_number: 67890,
        holder_name: String::from("Jesus"),
        balance: 0
    };

    bank_account1.deposit(5000);

    bank_account2.withdraw(1000);

    println!("Balance1: {}", bank_account1.balance);
    println!("Balance2: {}", bank_account2.balance);


}
