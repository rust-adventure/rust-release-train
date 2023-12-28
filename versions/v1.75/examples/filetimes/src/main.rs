use std::fs::{self, File, FileTimes};
use std::time::{Duration, SystemTime};

fn main() -> std::io::Result<()> {
    let past = SystemTime::now()
        .checked_sub(Duration::from_secs(157800000))
        .unwrap();
    dbg!(SystemTime::now(), past);

    let dest = File::options()
        .write(true)
        .open("examples/filetimes/src/file.txt")?;
    let times = FileTimes::new().set_modified(past);
    dest.set_times(times)?;
    Ok(())
}
