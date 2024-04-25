use command::Command;
use smol::io::{AsyncBufReadExt, AsyncRead, BufReader};
use smol::process::unix::CommandExt;
use smol::process::Command as SysCommand;
use smol::stream::StreamExt;
use snafu::prelude::*;

mod command;
use command::parser::{CommandParser, FeedResult};

pub struct Tsh {}

impl Tsh {
    /// Creates a new Tsh context
    pub fn new() -> Tsh {
        Tsh {}
    }

    /// Runs a Tsh program from code
    pub async fn run(&mut self, code: impl AsyncRead + Unpin) -> Result<(), Error> {
        let reader = BufReader::new(code);
        let mut lines = reader.lines();
        let mut parser = CommandParser::new();
        while let Some(line) = lines.next().await {
            if let FeedResult::Command(command) = parser.feed_line(&line?) {
                self.call(command).await?
            }
        }
        Ok(())
    }

    async fn call(&mut self, Command(command): Command) -> Result<(), Error> {
        if command.len() == 0 {
            return Err(Error::EmptyCommand);
        }

        let _status = SysCommand::new(&command[0])
            .args(&command[1..])
            .status()
            .await?;
        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    IO { error: smol::io::Error },
    EmptyCommand,
}

impl From<smol::io::Error> for Error {
    fn from(value: smol::io::Error) -> Self {
        Error::IO { error: value }
    }
}
