use std::fs::File;
use std::io::{self, Write, Read};

struct Person {
    name: String,
    age: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age: u32 = buffer.trim().parse().expect("Invalid number");

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);
}

struct Config {
    username: String,
    sid: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).expect("Could not open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");

        let mut lines = contents.lines();
        let username = lines.next().unwrap_or("Unknown").to_string();
        let sid = lines.next().unwrap_or("00000000").to_string();

        Config { username, sid }
    }
}

fn reading_from_file() {
    let path = "/Users/johnross/RustroverProjects/Spring2025Systems/src/assignment 3/config.txt";
    let config = Config::from_file(path); // ✅ Fix: Complete function call
    println!("📜 Name: {}", config.username);
    println!("🎓 SID: {}", config.sid);
}

fn create_config_file() {
    let path = "/Users/johnross/RustroverProjects/Spring2025Systems/src/assignment 3/config.txt";

    if let Ok(mut file) = File::create(path) {
        let content = "John Ross\n20355612";
        file.write_all(content.as_bytes()).expect("Failed to write to file");
        println!("✅ config.txt has been created at {}", path);
    } else {
        println!("❌ Failed to create config.txt");
    }
}

fn main() {
    create_config_file(); // Ensures the config file exists
    reading_from_console();
    reading_from_file();
}
