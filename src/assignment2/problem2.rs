
pub fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();  // ✅ Clone original string
    cloned.push_str("World!");   // ✅ Modify only the clone
    cloned
}


pub fn run() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);  // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
