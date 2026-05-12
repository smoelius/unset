use std::{
    env::{args_os, current_dir},
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::{Command, exit},
};

fn main() -> Result<(), Error> {
    let args = args_os().collect::<Vec<_>>();

    if args.len() <= 1 {
        return Ok(());
    }

    let path = find_unset_txt()?;

    let variables = read_to_string(path)?
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

fn find_unset_txt() -> Result<PathBuf, Error> {
    let mut candidate = Some(current_dir()?);

    while let Some(directory) = candidate {
        let path = directory.join("unset.txt");

        if path.is_file() {
            return Ok(path);
        }

        candidate = directory.parent().map(PathBuf::from);
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "could not find unset.txt in the current directory or any parent directory",
    ))
}
