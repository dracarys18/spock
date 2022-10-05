use std::ffi::OsStr;
use std::process::{Command, Output};

pub(crate) fn run<A, I>(command: &str, args: A) -> Result<Output, std::io::Error>
where
    A: IntoIterator<Item = I>,
    I: AsRef<OsStr>,
{
    Command::new(command).args(args).output()
}
