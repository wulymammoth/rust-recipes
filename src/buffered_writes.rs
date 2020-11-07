/// using println! has a performance impact as it flushes to terminal after every invocation
/// an alternative is to use buffered writes
use std::io::{self, Write};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the global stdout entity
    let stdout = io::stdout();

    // optional: wrap the handle in a buffer
    // - using a `BufWriter` also helps do lock and unlocking on `stdout`
    let mut handle = io::BufWriter::new(stdout);

    // `?` if we care about errors here
    writeln!(handle, "foo: {}", 42)?;
    Ok(())
}
