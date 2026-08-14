struct BankAccount {
	owner:String,
	balance:f64,
}

impl BankAccount{
	fn new(owner:&str)->BankAccount{
		BankAccount{
			owner:owner.to_string(),
			balance:0.0,
		}
}

	fn deposit(&mut self,amount:f64){
		self.balance = amount + self.balance
     }

	fn withdraw(&mut self,amount:f64)->bool{
		if self.balance>amount{
			self.balance= self.balance -amount;
            return true
        }
        else{
            return false }
    }

 	fn balance(&self)-> f64{
		self.balance
       }
    }

// Q: Why does deposit need &mut self but balance only needs &self?
// A: deposit modifies self.balance (writes to it), so it needs mutable access (&mut self).
//    balance only reads self.balance, so an immutable borrow (&self) is enough.

fn main() {
	let mut account = BankAccount::new("Deepak");

	account.deposit(500.0);
	println!("Balance after deposit 500: {}", account.balance());

	let r1 = account.withdraw(200.0);
	println!("Withdraw 200: {} | Balance: {}", r1, account.balance());

	let r2 = account.withdraw(1000.0);
	println!("Withdraw 1000: {} | Balance: {}", r2, account.balance());

	println!("Final → {} owes balance: {}", account.owner, account.balance());
}