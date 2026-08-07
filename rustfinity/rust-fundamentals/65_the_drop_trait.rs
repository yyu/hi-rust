/*
    # 65. The Drop Trait

    The `Drop` trait in Rust is used to run custom cleanup logic when a value goes out of scope. This is particularly useful for managing resources like files, network connections, or any other system resources that need to be explicitly released.

    When a value that implements `Drop` goes out of scope, the Rust runtime automatically calls the `drop` method, allowing you to define custom behavior for resource cleanup.

    In this challenge, you will implement a struct that represents a temporary file. The file should be automatically deleted when the struct is dropped, simulating the behavior of temporary files in real-world applications.

    ## Your Task

    1. Create `TempFile` struct with a method `new` that takes a file name as input, it must accept both `&str` and `String` types.
    2. Implement the `Drop` trait for `TempFile` to delete the file automatically when the struct goes out of scope.

    ## Hints

    If you're stuck, here are some hints to help you solve the challenge:

    <details>
        <summary>Click here to reveal hints</summary>

    - Use `std::fs::File` to create a temporary file.
    - Use `std::env::temp_dir()` to get the path for temporary files.
    - The `std::fs::remove_file` method can be used to delete files.
    - The `PathBuf` struct is helpful for managing file paths.
    - Use the `AsRef<str>` trait to allow flexible input types for the file name.
    - Implement the `Drop` trait for custom cleanup logic.

    </details>
*/

use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function
    pub fn new(filename: impl AsRef<str>) -> std::io::Result<Self> {
        let mut path = std::env::temp_dir();
        path.push(PathBuf::from(filename.as_ref()));
        std::fs::File::create(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        // unimplemented!()
        let _ = std::fs::remove_file(&self.path);
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
