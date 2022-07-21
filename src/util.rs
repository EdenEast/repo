use anyhow::{Context, Result};
use std::{
    borrow::Borrow,
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

pub fn make_path_buf<S: AsRef<str>>(s: S) -> Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}

pub fn read_content<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut content = String::new();
    File::open(&path)
        .context(format!("failed to open file: {:#?}", path))?
        .read_to_string(&mut content)
        .context(format!("failed to read file: '{:#?}'", path))?;

    Ok(content)
}

pub fn write_content<P, F>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut File) -> Result<()>,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write_fn(&mut file)
}

pub mod process {
    use anyhow::{Context, Result};
    use std::{
        io::{BufRead, BufReader},
        process::{Command, ExitStatus, Stdio},
    };

    pub fn inherit(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
    }

    pub fn piped(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command
    }

    pub fn null(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
        command
    }

    pub fn execute_command(command: &mut Command, prefix: String) -> Result<ExitStatus> {
        let mut child = command
            .spawn()
            .context("failed executing command as a child process")?;

        let stdout_child = if let Some(stdout) = child.stdout.take() {
            let pre = prefix.clone();
            Some(std::thread::spawn(move || forward_stdout(stdout, &pre)))
        } else {
            None
        };

        if let Some(stderr) = child.stderr.take() {
            forward_stdout(stderr, &prefix).context("could not forward stderr to stdout")?;
        }

        if let Some(child_thread) = stdout_child {
            child_thread
                .join()
                .expect("failed to join stdout child thread with main thread")?;
        }

        child.wait().map_err(Into::into)
    }

    fn forward_stdout<T>(read: T, prefix: &str) -> Result<()>
    where
        T: std::io::Read,
    {
        let mut buffer = BufReader::new(read);
        loop {
            let mut line = String::new();
            let result = buffer
                .read_line(&mut line)
                .context("could not read buffered line")?;
            if result == 0 {
                break;
            }

            // TODO: Have computed the larget string before calling this
            // but format does not allow formatting with dynamic variables.
            // This means that I can't format left based on the max_size
            let prefix = format!("{:>20.20} |", prefix);
            print!("{} {}", prefix, line);
        }

        Ok(())
    }
}

#[cfg(not(windows))]
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref().canonicalize().map_err(Into::into)
}

#[cfg(windows)]
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref()
        .canonicalize()
        .map_err(Into::into)
        .map(|path| {
            path.to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace("\\", "/")
        })
        .map(PathBuf::from)
}
