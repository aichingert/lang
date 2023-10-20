use std::io::{self, Write};

mod ast;
use ast::Parser;

fn repl() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        let mut buf = String::new();

        io::stdout().flush()?;
        stdin.read_line(&mut buf)?;
        buf = buf.trim().to_string();

        if buf == "q" {
            break;
        }

        let mut parser = Parser::new(&buf);

        println!("{}", parser.parse().evaluate());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    repl()?;

    Ok(())
}
