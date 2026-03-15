use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..5 {
        writeln!(handle, "line {i}")?;
    }
    Ok(())
}
