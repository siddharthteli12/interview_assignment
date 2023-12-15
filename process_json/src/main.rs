use std::{
    env,
    fs::File,
    io::{self, stdin, stdout, BufReader, BufWriter, Read, Write},
};

const BUFFER_SIZE: usize = 1024 * 1024;
pub fn process_json<R: Read, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    for byte in reader.bytes().flatten() {
        if byte == b';' {
            writer.write_all(&[b':']).expect("Unable to write to file");
        } else {
            writer.write_all(&[byte]).expect("Unable to write to file");
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Box<dyn Read> = if args.len() > 2 && args[1] == "--input" {
        Box::new(BufReader::with_capacity(
            BUFFER_SIZE,
            File::open(&args[2]).unwrap(),
        ))
    } else {
        Box::new(BufReader::with_capacity(BUFFER_SIZE, stdin()))
    };

    let output: Box<dyn Write> = if args.len() > 4 && args[3] == "--output" {
        Box::new(BufWriter::with_capacity(
            BUFFER_SIZE,
            File::create(&args[4]).unwrap(),
        ))
    } else {
        Box::new(BufWriter::with_capacity(BUFFER_SIZE, stdout()))
    };
    process_json(input, output).expect("Unable to process json file");
}
