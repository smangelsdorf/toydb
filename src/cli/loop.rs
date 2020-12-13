use std::io::{self, BufRead, Write};

use crate::cli::{
    command::handle_command, statement::handle_statement, Context, HandleInputOutcome,
};

use crate::storage::memory::Table;

pub fn cli_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut output = io::stdout();

    let mut buf = String::with_capacity(1024);

    let mut table = Table::new();

    loop {
        buf.clear();
        print_prompt(&mut output)?;
        input.read_line(&mut buf)?;

        let context = Context {
            output: &mut output,
            buf: &mut buf,
            table: &mut table,
        };

        match handle_input(context)? {
            HandleInputOutcome::Exit => break Ok(()),
            HandleInputOutcome::Continue => (),
        }
    }
}

fn handle_input(context: Context) -> io::Result<HandleInputOutcome> {
    if context.buf.starts_with("\\") {
        handle_command(context)
    } else {
        handle_statement(context)
    }
}

fn print_prompt(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.write(b"> ")?;
    stdout.flush()
}
