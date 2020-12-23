mod command;
pub mod r#loop;
mod statement;

use std::io::Write;

use crate::storage::memory::Table;

#[must_use]
enum HandleInputOutcome {
    Continue,
    Exit,
}

struct Context<'a> {
    output: &'a mut dyn Write,
    buf: &'a mut String,
    table: &'a mut Table,
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::r#loop::cli_loop;

    #[test]
    fn cli_interaction_test() {
        let input: &'_ [u8] = b"insert 1 abc test@example.com\nselect\n\\q\n";
        let mut output: Vec<u8> = Vec::with_capacity(256);

        cli_loop(input, &mut output).unwrap();

        let output_string = String::from_utf8(output).unwrap();
        assert_eq!(
            "> Success.\n> (1, abc, test@example.com)\n1 rows.\n> Bye!\n",
            output_string
        );
    }

    #[test]
    #[should_panic]
    fn insert_sanity_check_test() {
        let input: &'_ [u8] = b"insert 1 thisvalueiswaytoolongforthefielditsbeingputin \
            test@example.com\nselect\n\\q\n";

        let mut output: Vec<u8> = Vec::with_capacity(256);

        cli_loop(input, &mut output).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_full_table_test() {
        let mut input: String = String::with_capacity(60_000);
        for i in 0..1401 {
            input.extend(format!("insert {i} value{i} user{i}@example.com\n", i = i).chars());
        }
        input.extend("\\q\n".chars());

        let mut output = io::sink();

        cli_loop(input.as_bytes(), &mut output).unwrap();
    }
}
