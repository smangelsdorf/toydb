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
}
