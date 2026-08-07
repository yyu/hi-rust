/*
    # 35. Student Grades Tracker

    Now that we know about vectors and hashmaps, let’s tackle something more challenging. In this exercise, we’ll build a system to manage and analyze student grades. This challenge will help you practice working with structs, hashmaps, vectors, and methods while managing a collection of data.

    The focus here is on functionality. **Ignore error handling for this challenge**; assume the input is always valid.

    ## Your Task

    You need to define the `StudentGrades` struct, which contains a `HashMap` of student names (`String`) as keys and `Student` structs as values. Each `Student` struct should have the following fields:

    - `name`: The name of the student (`String`).
    - `grades`: A `Vec<u8>` to store the student's grades.

    Implement the following methods for the `StudentGrades` struct:

    1. **`add_student(name: &str)`:** Add a new student to the `HashMap`. If the student already exists, do nothing.
    2. **`add_grade(name: &str, grade: u8)`:** Add a grade to an existing student.
    3. **`get_grades(name: &str) -> &[u8]`:** Retrieve the grades of a student as an immutable reference.
    4. Since there’s no error handling, assume all inputs are valid, and keys will exist when accessed.

    ## Hints

    <details>
    <summary>Click here to reveal hints</summary>

    - Use the `HashMap` methods like `entry`, `insert`, and `get` to manage student data.

    </details>
*/

use std::collections::HashMap;

pub struct Student {
    // 1. Define the fields
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    // 2. Define the fields
    pub students: HashMap<String, Student>
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        // Implement here
        if self.students.get(name).is_none() {
            self.students.insert(
                name.to_string(),
                Student {
                    name: name.to_string(),
                    grades: Vec::new(),
                },
            );
        }
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // Implement here
        // self.students.get_mut(name).unwrap().grades.push(grade);
        if let Some(student) = self.students.get_mut(name) {
            student.grades.push(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
        // &self.students.get(name).unwrap().grades[..]
        if let Some(student) = self.students.get(name) {
            student.grades.as_slice()
        } else {
            &[]
        }
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
