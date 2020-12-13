use std::io;

use crate::cli::{Context, HandleInputOutcome};
use crate::query::{ast::Query, parser::parse_sql};
use crate::storage::memory::Row;

pub(in crate::cli) fn handle_statement(context: Context) -> io::Result<HandleInputOutcome> {
    match parse_sql(&context.buf) {
        Ok((_, Query::Select)) => {
            let mut n = 0;
            for Row {
                id,
                username,
                email,
            } in context.table.row_iter()
            {
                n += 1;
                write!(context.output, "({}, ", id)?;
                context.output.write(username)?;
                context.output.write(b", ")?;
                context.output.write(email)?;
                context.output.write(b")\n")?;
            }
            writeln!(context.output, "{} rows.", n)?;
        }
        Ok((_, Query::Insert(id, username, email))) => {
            context.table.insert((id, username, email));
            writeln!(context.output, "Success.")?;
        }
        Err(e) => writeln!(context.output, "Err: {:?}\n", e)?,
    }

    Ok(HandleInputOutcome::Continue)
}
