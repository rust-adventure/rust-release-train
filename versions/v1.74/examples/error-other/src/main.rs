use std::io::{self, Error};

fn main() -> io::Result<()> {
    let my_error = Error::other("some custom error");
    return Err(my_error);
}
