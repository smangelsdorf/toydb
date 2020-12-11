use std::io::{self, Write};

#[must_use]
enum HandleInputOutcome {
    Continue,
    Exit,
}

pub fn cli_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut buf = String::with_capacity(1024);

    loop {
        buf.clear();
        print_prompt(&mut stdout)?;
        stdin.read_line(&mut buf)?;

        match handle_input(&buf, &mut stdout)? {
            HandleInputOutcome::Exit => break Ok(()),
            HandleInputOutcome::Continue => (),
        }
    }
}

fn handle_input(buf: &str, io: &mut dyn Write) -> io::Result<HandleInputOutcome> {
    if buf == "\\q\n" {
        Ok(HandleInputOutcome::Exit)
    } else {
        io.write(b"Unexpected command: ")?;
        io.write(buf.as_bytes())?;
        io.write(b"\n")?;
        Ok(HandleInputOutcome::Continue)
    }
}

fn print_prompt(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.write(b"> ")?;
    stdout.flush()
}
