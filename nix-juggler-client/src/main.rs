use ::std::process::Command;

fn main() -> std::io::Result<()> {
    let child = Command::new("ls").arg("-l").arg("-a").spawn()?;

    Ok(())
}
