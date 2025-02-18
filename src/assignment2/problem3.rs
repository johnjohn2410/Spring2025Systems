
pub fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;  // ✅ Modify total via mutable reference
    }
}


pub fn run() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total sum: {}", total); // Should print: "Total sum: 5050"
}
