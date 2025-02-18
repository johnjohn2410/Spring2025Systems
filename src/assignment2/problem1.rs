
pub fn concat_strings(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2) // ✅ Returns a new string
}


pub fn run() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("Concatenated String: {}", result); // Should print: "Hello, World!"
}
