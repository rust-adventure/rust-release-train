use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let output = Command::new("whoami")
        // this line redirects the output from the command to the process' stdout
        .stdout(io::stdout())
        .output()?;
    dbg!(output.status);
    // this assert will fail if the output above is not redirected
    assert!(output.stdout.is_empty());

    let output = Command::new("ls")
        .arg("a-directory-that-doesnt-exist")
        // this line redirects the stderr from the command to the process' stderr
        .stderr(io::stderr())
        .output()?;
    dbg!(output.status);
    // this assert will fail if the output above is not redirected
    assert!(output.stderr.is_empty());

    Ok(())
}
