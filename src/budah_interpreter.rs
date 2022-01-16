pub mod string_handler;
mod io_handler;

const MEMORY_SIZE: usize = 0x10000;

pub struct BudahInterpreter
{
    memory: [u8; MEMORY_SIZE],
    index: u16
}

impl BudahInterpreter
{
    /// Returns a new BudahInterpreter instance
    pub fn new() -> BudahInterpreter
    {
        BudahInterpreter{memory: [0; MEMORY_SIZE], index: 0}
    }

    /// Will interpert the current line
    /// 
    /// # Arguments
    /// * `current_line` - the current line that we want to interpret
    pub fn interpert(&mut self, current_line: String)
    {
        let split_words = string_handler::split_words(current_line);
        
        for mut word in split_words
        {
            if word.ends_with('\r')
            {
                word.pop();
            }
            
            match word.as_str() {
                "Budah" => self.decrement_index(),
                "budaH" => self.increment_index(),
                "BUDAH" => self.read(),
                "bUdah" => self.decrement(),
                "budAh" => self.increment(),
                "budah" => self.print(),
                _ => ()
            }
        }
    }

    fn decrement_index(&mut self)
    {
        self.index -= 1;
    }

    fn increment_index(&mut self)
    {
        self.index += 1;
    }

    fn decrement(&mut self)
    {
        self.memory[self.index as usize] -= 1;
    }

    fn increment(&mut self)
    {
        self.memory[self.index as usize] += 1;
    }

    fn read(&mut self)
    {
        print!("Please enter a value: ");
        let value = io_handler::read_char();

        self.memory[self.index as usize] = value;
    }

    fn print(&self)
    {
        print!("{}", self.memory[self.index as usize] as char);
    }
}