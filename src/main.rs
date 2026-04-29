use std::io::{self, Write};

use cs_lang::lexer::Lexer;

fn ler_entrada(prompt: &str) -> io::Result<String> {
    let mut entrada = String::new();

    print!("{prompt}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut entrada)?;

    Ok(entrada.trim().to_string())
}

fn main() -> io::Result<()> {
    let code = ler_entrada("digite your code ")?;
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("{tokens:?}");

    Ok(())
}
