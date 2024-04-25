use std::env;

use smol::{fs::File, io::AsyncRead, Unblock};
use snafu::{prelude::*, Whatever};
use tsh::Tsh;

fn main() -> Result<(), Whatever> {
    smol::block_on(async {
        let args: Vec<_> = env::args().collect();
        let script_file_name = if args.len() > 2 { Some(&args[1]) } else { None };
        let script = if let Some(file_name) = script_file_name {
            Box::new(
                File::open(file_name)
                    .await
                    .whatever_context("Cannot open script file")?,
            ) as Box<dyn AsyncRead + Unpin>
        } else {
            Box::new(Unblock::new(std::io::stdin())) as Box<dyn AsyncRead + Unpin>
        };

        let mut tsh = Tsh::new();
        tsh.run(script)
            .await
            .whatever_context("Error during running script")?;
        Ok(())
    })
}
