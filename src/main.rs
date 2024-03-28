use std::io;
use std::io::{Read, Write};
use clap::Parser;
use arboard::Clipboard;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Write stdin to clipboard
    #[arg(short, long)]
    write: bool,
}

fn main() {
    let args = Args::parse();
    let mut clip = Clipboard::new().unwrap();

    if args.write {
        let mut buf = vec![];
        io::stdin().read_to_end(&mut buf).unwrap();
        io::stdout().write(&buf).unwrap();
        clip.set_text(String::from_utf8(buf).unwrap()).unwrap();
    } else {
        io::stdout().write(clip.get_text().unwrap().as_bytes()).unwrap();
    }
}
