use std::io;
use std::process::ExitCode;

use clap::Parser;
use rs_jpg2exif::stdin2jpg2exif2stdout;
use rs_jpg2exif::IMG_INPUT_SIZE_LIMIT_DEFAULT;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = IMG_INPUT_SIZE_LIMIT_DEFAULT)]
    input_img_bytes_max: u64,
}

fn sub() -> Result<(), io::Error> {
    let args = Args::parse();
    stdin2jpg2exif2stdout(args.input_img_bytes_max)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
