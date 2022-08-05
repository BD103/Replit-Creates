mod lex;
mod runner;
mod speak_map;
mod statement;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() {
    // Load script from given filename or main.emo.
    let script = if let Some(filename) = env::args().nth(1) {
        let path = Path::new(&filename);
        load_file(path).expect("Error loading file.")
    } else {
        let path = Path::new("./main.emo");
        load_file(path).expect("Error loading file.")
    };

    let tokens = lex::tokenize(script);

    let statements = statement::parse_tokens(tokens);

    let mut runner = runner::create_runner();

    runner.execute_statements(statements);
}
