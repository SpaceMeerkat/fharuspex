use binrw::{BinRead, BinResult};
use std::fs::File;
use std::io::{BufReader, SeekFrom};


#[binrw::parser(reader, endian)]
pub fn parse_chunk(offset: u8, size: u8) -> BinResult<Vec<u8>> {
    let mut chunk_buffer = Vec::new();

    match reader.seek(SeekFrom::Start(offset as u64)) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error starting bufreader at offset position: {}", offset);
            panic!("Terminating due to bufreader positional error: {}", err);
        }
    };

    reader.read_exact(&mut chunk_buffer); // Reads the exact number of bytes required to fill buffer

    let num_elements = size as usize / 4;

    for _ in 0..num_elements {
        let bytes = match <[u8; 4]>::read_options(reader, endian, ()) { // Read 4 bytes
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("Error parsing 4 bytes from buffer: {}", err);
                panic!("Terminating due to buffer value parsing.");
            }
        };
        
        // let value = i32::from_be_bytes(bytes);
        println!("Found i32 value of: {:?}", value);
        chunk_buffer.push(value);
    }

    Ok(chunk_buffer)
}

#[derive(BinRead)]
#[derive(Debug)]
#[br(big)]
pub struct DataChunk {
    #[br(parse with parse_chunk)]
    pub chunk: Vec<u8>,
}

pub fn open_data_chunk(file_path: &str, offset: u8, size: u8) -> BinResult<DataChunk> {
    let file = File::open(file_path)?;

    let fits_reader = BufReader::new(file);

    let data_chunk = match DataChunk::read(&mut fits_reader) {
        Ok(data_chunk) => data_chunk,
        Err(err) => {
            eprintln!("Error running DataChunk reading from file: `{}`", file_path);
            panic!("Terminating DataChunk reading with error: {}", err);
        },
    };
}