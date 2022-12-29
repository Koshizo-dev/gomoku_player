use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct Runtime {
    pub process: Child,
    pub stdout: BufReader<ChildStdout>,
    pub stdin: BufWriter<ChildStdin>,
}

impl Runtime {
    pub fn init(path: &str) -> Result<Self, String> {
        let mut process = Command::new(&path)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect(&format!("failed to start process [{}]", path));

        let stdout = BufReader::new(
            process
                .stdout
                .take()
                .expect("failed to capture standard output"),
        );

        let stdin = BufWriter::new(
            process
                .stdin
                .take()
                .expect("failed to capture standard input"),
        );

        Ok(Runtime {
            process,
            stdout,
            stdin,
        })
    }

    pub fn read(&mut self) -> Result<String, Error> {
        let mut content = String::new();

        match self.stdout.read_line(&mut content) {
            Ok(_) => Ok(content),
            Err(err) => Err(err),
        }
    }

    pub fn write(&mut self, target: String) -> Result<(), Error> {
        writeln!(self.stdin, "{}", target)
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdin.flush()
    }
}
