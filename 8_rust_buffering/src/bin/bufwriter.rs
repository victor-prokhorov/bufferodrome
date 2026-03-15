use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut chars = 0;
    for i in 0..5 {
        let s = format!("buffered line {i}");
        chars += s.len();
        writeln!(writer, "{s}")?;
    }
    assert!(writer.capacity() <= chars);
    writer.flush()?;
    Ok(())
}
