use arboard::Clipboard;
use clap::Parser;
use std::io;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Write stdin to clipboard
    #[arg(short, long)]
    write: bool,

    /// Log additional error details
    #[arg(short, long)]
    verbose: bool,
}

enum Error {
    ClipboardError(arboard::Error),
    ConversionError(std::string::FromUtf8Error),
    IoError(std::io::Error),
}

fn main() {
    let args = Args::parse();
    let mut clip = Clipboard::new().unwrap();

    let result = if args.write {
        try_write(&mut clip)
    } else {
        try_read(&mut clip)
    };

    if let Err(e) = result {
        handle_error(e, args.verbose);
    }
}

fn try_read(clip: &mut Clipboard) -> Result<(), Error> {
    let text = clip.get_text().map_err(|e| Error::ClipboardError(e))?;

    io::stdout()
        .write(&text.as_bytes())
        .map(|_| ())
        .map_err(|e| Error::IoError(e))
}

fn try_write(clip: &mut Clipboard) -> Result<(), Error> {
    let mut buf = Vec::new();

    io::stdin()
        .read_to_end(&mut buf)
        .map_err(|e| Error::IoError(e))?;

    let str = String::from_utf8(buf).map_err(|e| Error::ConversionError(e))?;

    clip.set_text(&str).map_err(|e| Error::ClipboardError(e))?;

    io::stdout()
        .write(&str.as_bytes())
        .map(|_| ())
        .map_err(|e| Error::IoError(e))
}

fn handle_error(e: Error, verbose: bool) {
    let msg = if !verbose {
        match e {
            Error::ClipboardError(_) => "clipboard error",
            Error::ConversionError(_) => "utf8 conversion error",
            Error::IoError(_) => "io error",
        }
    } else {
        match e {
            Error::ClipboardError(err) => match err {
                arboard::Error::ClipboardOccupied => "clipboard not available",
                arboard::Error::ContentNotAvailable => "content not available",
                arboard::Error::ConversionFailure => "could not convert clipboard content",
                arboard::Error::ClipboardNotSupported => "clipboard not supported",
                arboard::Error::Unknown { description } => {
                    eprintln!("clipboard error: {}", description);
                    return;
                }
                _ => "unknown clipboard error",
            },
            Error::IoError(err) => {
                eprintln!("io error: {}", err);
                return;
            }
            Error::ConversionError(err) => {
                eprintln!("utf8 error: {}", err);
                return;
            }
        }
    };

    eprintln!("error: {}", msg);
}
