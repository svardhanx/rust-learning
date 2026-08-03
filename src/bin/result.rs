enum WithdrawError {
    InvalidAmount(String),
    InsufficientAmount,
}

fn withdraw(balance: u32, amount: u32) -> Result<u32, WithdrawError> {
    if amount == 0 {
        let formatted_string = format!("You requested {amount} rupees which is invalid.");
        Err(WithdrawError::InvalidAmount(String::from(formatted_string)))
    } else if amount > balance {
        Err(WithdrawError::InsufficientAmount)
    } else {
        Ok(balance - amount)
    }
}

fn main() {
    let amount: u32 = 0;
    let balance: u32 = 1000;

    match withdraw(balance, amount) {
        Err(WithdrawError::InsufficientAmount) => {
            println!("You have insufficient amount to withdraw: {amount} rupees.")
        }

        Err(WithdrawError::InvalidAmount(msg)) => {
            println!("{msg}")
        }

        Ok(value) => {
            println!("Remaining balance: {value} rupees.")
        }
    }
}
