use std::ffi::OsStr;
use std::fs;
use std::io::Write;
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
    assert_ne!(args.input_path, args.output_path);

    let ext = args.input_path.extension().and_then(OsStr::to_str);

    match ext {
        Some("csv") => extract_from_csv(args.input_path, args.output_path),
        Some(_) => println!("Unsupported file type!"),
        None => eprintln!("File has no extension!"),
    }
}

fn extract_from_csv(input_file: PathBuf, output_file: PathBuf) {
    let input_data = fs::read_to_string(input_file).unwrap();
    let mut output_data = fs::File::create(&output_file).unwrap();

    writeln!(
        output_data,
        "uint8_t {}[256] = {{",
        output_file.file_stem().and_then(OsStr::to_str).unwrap()
    )
    .unwrap();
    for (index, line) in input_data.lines().enumerate() {
        if (index + 1) % 16 != 0 {
            write!(output_data, "0x{}, ", line).unwrap();
        } else {
            writeln!(output_data, "0x{}, ", line).unwrap();
        }
    }
    writeln!(output_data, "}};").unwrap();
}
