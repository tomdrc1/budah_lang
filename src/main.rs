mod file_handler;
mod budah_interpreter;

use std::env;
use budah_interpreter::string_handler;

fn main()
{
    let args = env::args().collect::<Vec<String>>();
    
    let file_path = &args[1];
    let file_content = file_handler::read_file(file_path.to_string());
    let lines = string_handler::split_newlines(file_content);

    let mut interpreter = budah_interpreter::BudahInterpreter::new();

    for line in lines
    {
        interpreter.interpert(line);
    }
}
