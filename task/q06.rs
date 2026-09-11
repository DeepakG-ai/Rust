// ============================================================================
// ## Q6 — `&mut self`
//
// ```rust
// struct BankAccount {
//     owner: String,
//     balance: f64,
// }
// ```
//
// Methods:
// - `fn new(owner: &str) -> BankAccount` — starts at 0.0
// - `fn deposit(&mut self, amount: f64)`
// - `fn withdraw(&mut self, amount: f64) -> bool` — refuse and return `false` if
//   the balance is too low, otherwise subtract and return `true`
// - `fn balance(&self) -> f64`
//
// In `main`, declare it as `let mut account = ...`, deposit 500, withdraw 200,
// try to withdraw 1000, print the result of each attempt and the final balance.
//
// Question to answer in a comment:
// - Why does `deposit` need `&mut self` but `balance` only needs `&self`?
// ============================================================================

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