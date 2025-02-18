pub fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

pub fn run() {
    let secret = 7; // Secret number
    let mut guess = 5; // Simulated user input
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The number was {}. Attempts: {}", secret, attempts);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }


        guess += 1;
    }
}
