use smol::io::{AsyncBufReadExt, AsyncRead, BufReader};
use smol::stream::StreamExt;
use snafu::prelude::*;

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
        while let Some(line) = lines.next().await {
            println!("{}", line?)
        }
        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    IO { error: smol::io::Error },
}

impl From<smol::io::Error> for Error {
    fn from(value: smol::io::Error) -> Self {
        Error::IO { error: value }
    }
}
