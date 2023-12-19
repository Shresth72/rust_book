use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    Ok(())
}

fn function1() -> io::Result<()> {
    Ok(())
}