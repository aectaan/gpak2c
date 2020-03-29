use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gpak2c",
    about = "GPAK output converter from *.hex, *.txt or *.csv into *.c files"
)]
struct InputArgs {
    #[structopt(short = "i", long = "input")]
    input_path: PathBuf,
    #[structopt(short = "o", long = "output")]
    output_path: PathBuf,
}

fn main() {
    let args = InputArgs::from_args();

    let ext = args.input_path.extension().and_then(OsStr::to_str);

    match ext {
        Some("csv") => extract_from_csv(args.input_path),
        Some(_) => println!("Unsupported file type!"),
        None => eprintln!("File has no extension!"),
    }
}

fn extract_from_csv(input_file: PathBuf) {
    let input_data = fs::read_to_string(input_file);
    println!("{:?}", input_data);
}
