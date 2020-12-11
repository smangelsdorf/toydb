use std::io;

use crate::cli::{Context, HandleInputOutcome};

pub(in crate::cli) fn handle_command(context: Context) -> io::Result<HandleInputOutcome> {
    if context.buf == "\\q\n" {
        context.output.write(b"Bye!\n")?;
        Ok(HandleInputOutcome::Exit)
    } else {
        context.output.write(b"Unexpected command: ")?;
        context.output.write(context.buf.as_bytes())?;
        context.output.write(b"\n")?;
        Ok(HandleInputOutcome::Continue)
    }
}
