mod problem3;
mod problem4;

fn main() {
    println!("Main thread starting...");

    println!("\n--- Running Problem 3 ---");
    problem3::run();

    println!("\n--- Running Problem 4 ---");
    problem4::run();

    println!("\nMain thread finished.");
}