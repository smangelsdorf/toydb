use std::io::{self, Write};

pub fn cli_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut buf = String::with_capacity(1024);

    loop {
        buf.clear();
        print_prompt(&mut stdout)?;
        stdin.read_line(&mut buf)?;

        if buf == "\\q\n" {
            break Ok(());
        } else {
            stdout.write(b"Unexpected command: ")?;
            stdout.write(buf.as_bytes())?;
            stdout.write(b"\n")?;
        }
    }
}

fn print_prompt(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.write(b"> ")?;
    stdout.flush()
}
