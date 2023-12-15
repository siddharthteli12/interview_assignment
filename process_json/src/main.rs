use std::{
    env,
    time::Instant,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

const BUFFER_SIZE: usize = 8192;

// Iter & replace byte for given buffer size.
fn process_json_chunk(chunk: &[u8]) -> Vec<u8> {
    chunk
        .iter()
        .map(|&byte| if byte == b';' { b':' } else { byte })
        .collect()
}

// Process json from reader & write to writer.
fn process_json<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        let processed_chunk = process_json_chunk(&buffer[..bytes_read]);
        writer.write_all(&processed_chunk)?;
    }
    Ok(())
}

fn main() {
    // Collect cli args
    let args: Vec<String> = env::args().collect();

    // Input is a file or stdin
    let input: Box<dyn Read> = if args.len() > 2 && args[1] == "--input" {
        Box::new(BufReader::with_capacity(
            BUFFER_SIZE,
            File::open(&args[2]).unwrap(),
        ))
    } else {
        Box::new(BufReader::with_capacity(BUFFER_SIZE, io::stdin()))
    };

    // Output is a file or stdout
    let output: Box<dyn Write> = if args.len() > 4 && args[3] == "--output" {
        Box::new(BufWriter::with_capacity(
            BUFFER_SIZE,
            File::create(&args[4]).unwrap(),
        ))
    } else {
        Box::new(BufWriter::with_capacity(BUFFER_SIZE, io::stdout()))
    };
    let now = Instant::now();
    process_json(input, output).expect("Unable to process json file");
    println!("Time elapsed - {:?}", now.elapsed());
}
