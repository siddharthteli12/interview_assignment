use std::{
    env,
    fs::File,
    io::{self, stdin, stdout, BufReader, BufWriter, Read, Write},
};

const BUFFER_SIZE: usize = 1024 * 8;

// Process json from input & write to output.
pub fn process_json<R: Read, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    for byte in reader.bytes().flatten() {
        let result = if byte == b';' { b':' } else { byte };
        writer.write(&[result])?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Collect cli args
    let args: Vec<String> = env::args().collect();

    // Input is a file or stdin
    let input: Box<dyn Read> = if args.len() > 2 && args[1] == "--input" {
        Box::new(BufReader::with_capacity(
            BUFFER_SIZE,
            File::open(&args[2]).unwrap(),
        ))
    } else {
        Box::new(BufReader::with_capacity(BUFFER_SIZE, stdin()))
    };

    // Output is a file or stdout
    let output: Box<dyn Write> = if args.len() > 4 && args[3] == "--output" {
        Box::new(BufWriter::with_capacity(
            BUFFER_SIZE,
            File::create(&args[4]).unwrap(),
        ))
    } else {
        Box::new(BufWriter::with_capacity(BUFFER_SIZE, stdout()))
    };

    process_json(input, output)?;
    Ok(())
}
