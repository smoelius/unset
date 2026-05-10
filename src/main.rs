use std::{
    env::args_os,
    fs::read_to_string,
    io::{Error, ErrorKind},
    process::{Command, exit},
};

fn main() -> Result<(), Error> {
    let args = args_os().collect::<Vec<_>>();

    if args.len() <= 1 {
        return Ok(());
    }

    let variables = read_to_string("unset.txt")?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect::<Vec<_>>();

    for variable in &variables {
        if variable.contains('=') || variable.contains('\0') {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid environment variable name in unset.txt: {variable}"),
            ));
        }
    }

    let mut command = Command::new(&args[1]);
    command.args(&args[2..]);

    for variable in &variables {
        command.env_remove(variable);
    }

    let status = command.status()?;

    if let Some(code) = status.code() {
        exit(code);
    }

    exit(1);
}
