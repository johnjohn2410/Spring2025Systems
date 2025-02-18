mod assignment2; // ✅ Only include Assignment 2

fn main() {
      println!("Rust Borrowing and Ownership Practice Problems");

      println!("\nProblem 1: String Concatenation");
      assignment2::problem1::run();

      println!("\nProblem 2: Clone and Modify");
      assignment2::problem2::run();

      println!("\nProblem 3: Mutable Reference Sum");
      assignment2::problem3::run();
}
