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
                let (username_trimmed, _) = partition_point(username, |b| *b == 0);
                let (email_trimmed, _) = partition_point(email, |b| *b == 0);
                n += 1;
                write!(context.output, "({}, ", id)?;
                context.output.write(username_trimmed)?;
                context.output.write(b", ")?;
                context.output.write(email_trimmed)?;
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

// Not yet stable in Rust stdlib. This is a slice method in nightly Rust.
fn partition_point<T, P>(slice: &[T], mut pred: P) -> (&[T], &[T])
where
    P: FnMut(&T) -> bool,
{
    for i in 0..slice.len() {
        if pred(&slice[i]) {
            return slice.split_at(i);
        }
    }

    slice.split_at(slice.len())
}
