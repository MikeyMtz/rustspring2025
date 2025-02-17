fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 7; // Hardcoded secret number
    let mut attempts = 0;
    let mut guess = 0; // Simulated user input (change this to test different values)

    loop {
        attempts += 1;
        guess += 1; // Simulating user incrementing guesses
        
        let result = check_guess(guess, secret_number);
        
        if result == 0 {
            println!("Correct! The secret number is {}.", secret_number);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }
    
    println!("It took {} attempts to guess correctly.", attempts);
}
