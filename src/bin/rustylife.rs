#![forbid(unsafe_code)]
use std::{io::Write, process::ExitCode};
fn main() -> ExitCode {
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(|v| v.into_string())
        .collect();
    let result = args
        .map_err(|_| "arguments must be valid Unicode".to_owned())
        .and_then(|args| rustylife::cli::run(&args));
    match result {
        Ok(text) => match std::io::stdout().write_all(text.as_bytes()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("rustylife: {e}");
                ExitCode::from(1)
            }
        },
        Err(error) => {
            eprintln!("rustylife: {error}");
            ExitCode::from(2)
        }
    }
}
