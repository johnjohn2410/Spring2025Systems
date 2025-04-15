// Task 1: Basic Closure
fn task1() {
    let operation = |a: i32, b: i32| a * b;
    println!("Result: {}", operation(10, 5));
}

// Task 2: Environment Capture
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Counter: {}", tracker);
    };

    update();
    update();
}

// Task 3: Vector Transformation (using map and collect)
fn process_vector_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Task 3: Vector Transformation (using for loop)
fn process_vector_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

// Task 5: Lazy Computation
use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let v = (self.computation)();
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn main() {
    println!("--- Task 1 ---");
    task1();

    println!("\n--- Task 2 ---");
    track_changes();

    println!("\n--- Task 3 ---");
    let numbers = vec![1, 2, 3];
    let doubled = process_vector_map(numbers.clone(), |x| x * 2);
    let replaced = process_vector_loop(numbers, |x| if x > 2 { 0 } else { x });
    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);

    println!("\n--- Task 5 ---");
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
