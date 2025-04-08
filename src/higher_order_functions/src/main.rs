// In class Assignment
// Create a struct Student (major)
struct Student {
    major: String,
}

// First Order Function: assign_major
fn assign_major(s: &mut Student, major_declared: String) {
    s.major = major_declared;
}

// Higher Order Function: takes a function pointer as behavior
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) {
    for student in &mut collection {
        behavior(student, String::from("Computer Science"));
    }

    // Print results to show it worked
    for (i, student) in collection.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}

fn main() {
    // Create a vector of students with empty majors
    let students = vec![
        Student { major: String::from("") },
        Student { major: String::from("") },
        Student { major: String::from("") },
    ];

    // Call update_majors with the assign_major function
    update_majors(students, assign_major);
}
