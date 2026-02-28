//Building a student profiler for a Rust-based academic system.
//Each student profile must contain: student ID, Age, GPA, Active Status, Grade letter

fn main() {
    let student_id: u32 = 10189;
    let age: u32 = 21;
    let gpa: f64 = 4.157;
    let active_status: bool = true;
    let grade: char = 'A';

    let student_profile = (student_id, age, gpa, active_status, grade);

    println!("Student ID: {}", student_profile.0);
    println!("Student Age: {}", student_profile.1);
    println!("GPA: {:.2}", student_profile.2);
    println!("Active Status: {}", student_profile.3);
    println!("Grade: {}", student_profile.4);
}