// Now, this session entails building a student profile initializer,
// meant for a Rust-based academic system.
//Project constraints: Each student profile must contain: Student ID number, Age, GPA, 
// Active Status, Grade Letter.

fn main() {
    let student_id: u32 = 11007; // Unsigned integer, non-negative integer, annotated not casted.
    let age: u32 = 19;
    let gpa: f64 = 3.78;
    let active_status: bool = true;
    let grade: char = 'A';

    let student_profile = (student_id, age, gpa, active_status, grade);

    println!("Student ID: {}", student_profile.0);
    println!("Age: {}", student_profile.1);
    println!("GPA: {:.2}", student_profile.2);
    println!("Active Status: {}", student_profile.3);
    println!("Grade: {}", student_profile.4);
}