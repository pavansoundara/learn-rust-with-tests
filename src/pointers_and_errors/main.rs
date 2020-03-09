type Bitcoin = i64;

struct Wallet {
    balance: Bitcoin,
}

impl Wallet {
    fn deposit(&mut self, amount: Bitcoin) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: Bitcoin) {
        self.balance -= amount;
    }

    fn balance(&self) -> Bitcoin {
        return self.balance;
    }
}

fn main() {
    let mut wallet = Wallet{balance: 0};
    wallet.deposit(10);
    wallet.withdraw(5);
    println!("{}", wallet.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut wallet = Wallet{balance: 0};
        wallet.deposit(10);
        assert_eq!(wallet.balance(), 10);
    }

    #[test]
    fn test_withdraw() {
        let mut wallet = Wallet{balance: 10};
        wallet.withdraw(5);
        assert_eq!(wallet.balance(), 5);
    }
}
