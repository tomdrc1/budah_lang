/// Returns the string split to lines.
/// 
/// # Arguments
/// 
/// * `content` - a string that we want to split it's new lines.
/// 
/// # Example
/// ```
/// fn main()
/// {
///     let splitted_lines = split_newlines("test1\ntest2\ntest3");
///     // Will return a vector containing ["test1", "test2", "test3"]
/// } 
pub fn split_newlines(content: String) -> Vec<String>
{
    let split_content = content.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    split_content
}

/// Returns a string split into words (every space)
/// 
/// # Arguments
/// 
/// * `content` - a string that we want to split it's words.
/// 
/// # Example
/// ```
/// fn main()
/// {
///     let splitted_words = split_words("test1 test2 test3");
///     // Will reutnr  avector containing ["test1", "test2", "test3"]
/// }
pub fn split_words(content: String) -> Vec<String>
{
    let split_words = content.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();

    split_words
}