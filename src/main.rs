use std::fs::File;
use std::io::Write;

use castle::render;

extern crate clap;
use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about)] //, long_about = None)]
struct Args {
    #[arg()]
    pub path: String,
    #[arg()]
    pub output: Option<String>,
}
fn main() {
    let args: Args = Args::parse();

    let file_out: String = args.output.unwrap_or("out.txt".into());
    let text: String = render(args.path.as_str(), [].to_vec());
    if file_out == "-" {
        println!("{}", text);
    } else {
        let mut file: File = File::create(file_out).unwrap();

        let _ = file.write_all(text.as_bytes());
    }
}
