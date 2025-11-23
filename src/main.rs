use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
struct Args {
    // Path to the .onnx file to be parsed
    #[arg(short, long)]
    input_path: PathBuf,

    // Path at which to place the generated module
    #[arg(short, long)]
    output_path: PathBuf
}

pub fn main() {
    let args = Args::parse();
    let graph = onnx_ir::parse_onnx(args.input_path.as_path());

    match std::fs::File::create(args.output_path) {
        Err(e) => { eprintln!("Error creating output file: {:?}", e); },
        Ok(file) => {
            // TODO: everything
            ()
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
