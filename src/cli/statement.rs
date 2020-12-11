use std::io;

use crate::cli::{Context, HandleInputOutcome};
use crate::query::{ast::Query, parser::parse_sql};

pub(in crate::cli) fn handle_statement(context: Context) -> io::Result<HandleInputOutcome> {
    match parse_sql(&context.buf) {
        Ok((_, Query::Select)) => writeln!(context.output, "This is where we would do a select.")?,
        Ok((_, Query::Insert)) => writeln!(context.output, "This is where we would do an insert.")?,
        Err(e) => writeln!(context.output, "Err: {:?}\n", e)?,
    }

    Ok(HandleInputOutcome::Continue)
}
