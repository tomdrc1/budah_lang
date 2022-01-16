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
}