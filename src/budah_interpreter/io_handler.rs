use std::io::Read;
use std::io;

/// Retruns a single byte that is being read from stdin
pub fn read_char() -> u8
{
    let byte = io::stdin().bytes().next().expect("no byte read").ok().unwrap();
    
    byte
}