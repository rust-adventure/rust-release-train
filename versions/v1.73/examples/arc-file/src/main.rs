use std::io::Read;
use std::{fs::File, sync::Arc};
fn main() -> std::io::Result<()> {
    // when run from the workspace root, the file is at
    let mut file =
        File::open("./examples/arc-file/src/main.rs")?;
    let mut file = Arc::new(file);
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("main.rs to be reachable");
    dbg!(buf);
    Ok(())
}
