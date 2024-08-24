/* compression shrinks file sizes to save storage space and speed up transfers, while decompression reverses this process to make the original file usable again. */

extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        /* this eprint are used to print error messages or diagonostic information to stderr */
        eprint!("Using: `source`, `target`");
        return;
    }

    /* helps read file, comming from connand line arguments */
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    /* this line to create output file */
    let output = File::create(args().nth(2).unwrap()).unwrap();
    /* This encoder will take the input data (e.g., a 34 MB PDF file) and compress it using the GZIP algorithm. */
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    /* this is copying our input data, to our output. */
    /* input:: input stream from which the uncompressed data will be read */
    /* output:: GzEncoder instance which is the output stream where the compress data will be
    writeen */
    copy(&mut input, &mut encoder).unwrap();

    /* is finalizing the compression process and getting the final compressed data. */
    /* encoder.finish() :: complets the compression, It ensures all data compressed and writeen to the output */
    /* unwrap() :: checks if the compression was successful */
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len: {:?}", output.metadata().unwrap().len());

    println!("Elapsed: {:?}", start.elapsed());
}
