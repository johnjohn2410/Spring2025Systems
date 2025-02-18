
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}


pub fn run() {  // ✅ Ensure `run()` is called in `main.rs`
    let numbers = [12, 5, 8, 15, 21, 9, 30, 45, 10, 27];

    // Analyze numbers
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {  // ✅ Now `is_even()` is used
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    // Calculate sum using while loop
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);

    // Find the largest number
    let mut max = numbers[0];
    for &num in numbers.iter() {
        if num > max {
            max = num;
        }
    }
    println!("Largest number: {}", max);
}
