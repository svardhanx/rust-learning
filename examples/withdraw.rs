enum WithdrawError {
    InvalidAmount(String),
    InsufficientFunds(String),
    NegativeAmount(String),
    NegativeBalance(String),
    EmptyBalance(String),
}

fn withdraw(balance: i32, amount: i32) -> Result<i32, WithdrawError> {
    if amount == 0 {
        return Err(WithdrawError::InvalidAmount(String::from(
            "Amount requested is zero. Cannot withdraw.",
        )));
    }

    if balance == 0 {
        return Err(WithdrawError::EmptyBalance(String::from(
            "Your account balance is zero. Operation aborted",
        )));
    }

    if balance < 0 {
        return Err(WithdrawError::NegativeBalance(String::from(
            "Your account has negative balance. Operation aborted",
        )));
    }

    if amount > balance {
        return Err(WithdrawError::InsufficientFunds(String::from(format!(
            "Cannot draw {amount} rupees since you only have {balance} rupees in your account"
        ))));
    }

    if amount < balance {
        return Err(WithdrawError::NegativeAmount(String::from(
            "This operation leads to negative balance in your account. Operation aborted",
        )));
    }

    Ok(balance - amount)
}

fn main() {
    match withdraw(100, 100) {
        Ok(balance) => println!("Remaining balance: {balance}"),

        Err(WithdrawError::InsufficientFunds(err_msg)) => {
            eprintln!("Insufficient funds error - {err_msg}")
        }

        Err(WithdrawError::InvalidAmount(err_msg)) => {
            eprintln!("Invalid amount error - {err_msg}")
        }

        Err(WithdrawError::NegativeAmount(err_msg)) => {
            eprintln!("Negative Amount error - {err_msg}")
        }

        Err(WithdrawError::EmptyBalance(err_msg)) => {
            eprintln!("Empty Balance error - {err_msg}")
        }

        Err(WithdrawError::NegativeBalance(err_msg)) => {
            eprintln!("Negative Balance error - {err_msg}")
        }
    }
}
