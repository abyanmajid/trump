// TESTING STUFF
use std::fs;
use trump::lexer::Lexer;

const DEBUG_LEXER: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_code = if let Some(file_path) = std::env::args().nth(1) {
        fs::read_to_string(file_path)?
    } else {
        // Default to reading from `tests/test.trump` if no file is provided
        println!("No source file provided. Reading from 'test_files/test.trump'...");
        fs::read_to_string("test_files/test.trump")?
    };

    if DEBUG_LEXER {
        let mut lexer = Lexer::new(source_code);
        while lexer.current_char.is_some() {
            println!("{:?}", lexer.next_token());
        }
    }

    Ok(())
}