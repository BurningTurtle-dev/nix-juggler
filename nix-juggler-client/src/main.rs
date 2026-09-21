use std::collections::VecDeque;
use std::env::args;
use std::process::{Command, Stdio};

fn spawn_writer(args: VecDeque<String>) -> std::io::Result<std::process::Child> {
    // Locate the writer binary next to this binary
    let exe = std::env::current_exe()?;
    let writer_path = exe.with_file_name("nix-juggler-writer");

    let child = Command::new(writer_path)
        .args(&args) // .args() takes IntoIterator<Item = AsRef<OsStr>>, Vec<String> works directly
        .stdin(Stdio::piped())
        //.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}

fn main() {
    let mut args: VecDeque<String> = args().collect();
    args.pop_front();

    match spawn_writer(args) {
        Ok(mut child) => match child.wait() {
            Ok(status) if !status.success() => eprintln!("writer exited with: {status}"),
            Err(e) => eprintln!("failed to wait on writer: {e}"),
            _ => {}
        },
        Err(e) => eprintln!("failed to spawn writer: {e}"),
    }
}
