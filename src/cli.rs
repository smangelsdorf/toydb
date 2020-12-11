mod command;
pub mod r#loop;
mod statement;

use std::io::Write;

#[must_use]
enum HandleInputOutcome {
    Continue,
    Exit,
}

struct Context<'a> {
    output: &'a mut dyn Write,
    buf: &'a mut String,
}
