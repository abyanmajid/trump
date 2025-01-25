use std::fs;
use std::io::Write;
use trump::lexer::Lexer;
use trump::parser::Parser;

const DEBUG_LEXER: bool = true;
const DEBUG_PARSER: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_code = if let Some(file_path) = std::env::args().nth(1) {
        fs::read_to_string(file_path)?
    } else {
        println!("No source file provided. Reading from 'test_files/test.trump'...");
        fs::read_to_string("test_files/test.trump")?
    };

    if DEBUG_LEXER {
        let mut lexer = Lexer::new(&source_code);
        while lexer.current_char.is_some() {
            println!("{:?}", lexer.next_token());
        }
    }

    if DEBUG_PARSER {
        let mut parser = Parser::new(Lexer::new(&source_code));

        let program = parser.parse_program();

        fs::create_dir_all("debug")?;

        let mut file = fs::File::create("debug/ast.json")?;
        let json = serde_json::to_string_pretty(&program.json())?;
        file.write_all(json.as_bytes())?;

        println!("Wrote AST to debug/ast.json successfully");
    }

    Ok(())
}
