// In-class Assignment
// Create a struct Student (major)
struct Student {
    major: String,
}

// First Order function: assign_major(student, major_declared)
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher Order function update_majors
fn update_majors(
    collection: &mut Vec<Student>,
    majors: Vec<String>,
    behavior: fn(&mut Student, String),
) {
    for (student, major) in collection.iter_mut().zip(majors.into_iter()) {
        behavior(student, major);
    }
}

fn main() {
    // Create a vector of 3 students with undeclared major
    let mut students = vec![
        Student {major: "Undeclared".to_string()},
        Student {major: "Undeclared".to_string()},
        Student {major: "Undeclared".to_string()},
    ];

    // Majors to be assigned
    let major_list = vec![
        "Electrical Engineering".to_string(),
        "Computer Science".to_string(),
        "Computer Engineering".to_string(),
    ];

    println!("Before update:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }

    // Update all students' majors
    update_majors(&mut students, major_list, assign_major);

    println!();
    println!("After update:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}