use std::fs;

/// Returns the contents of a file.
/// 
/// # Argumnets
/// 
/// * `file_path` - The path to the file on the system.
/// 
/// # Example
/// ```
/// fn main()
/// {
///     let file_content = read_file("test.txt".to_string());
/// }
/// ```
pub fn read_file(file_path: String) -> String
{
    let file_contents =  fs::read_to_string(file_path).expect("Something went wrong reading the file");

    file_contents
}