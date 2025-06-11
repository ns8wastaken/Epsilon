use std::io::{self, BufRead, Write};

pub struct UciIO {
    pub stdin: io::StdinLock<'static>,
    pub stdout: io::Stdout,
}

impl UciIO {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout()
        }
    }

    pub fn input(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.stdin.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some(line.trim().to_string()),
            Err(_) => None,
        }
    }

    pub fn out(&mut self, msg: &str) {
        writeln!(self.stdout, "{}", msg).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn outfmt(&mut self, args: std::fmt::Arguments) {
        self.stdout.write_fmt(args).unwrap();
        self.stdout.write_all(b"\n").unwrap();
        self.stdout.flush().unwrap();
    }
}
