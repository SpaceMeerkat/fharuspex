use binrw::{BinRead, BinResult};
use std::fs::File;
use std::io::{BufReader, SeekFrom};

#[binrw::parser(reader, endian)]
pub fn parse_chunk(args: ChunkArgs) -> BinResult<Vec<f64>> { // numpy array is using float64-bit
    let mut chunk_vector = Vec::new();

    reader.seek(SeekFrom::Start(args.offset))?;

    for _ in 0..args.size {
        let value = f64::read_options(reader, endian, ())?;
        // BinRead automatically advances the BufReader position (8 bits [64 floating point value] at a time)
        chunk_vector.push(value);
    }

    Ok(chunk_vector)
}

#[derive(BinRead, Debug)]
#[br(big, import(args: ChunkArgs))] // Importing the offset and size from ChunkArgs struct
pub struct DataChunk {
    #[br(parse_with = parse_chunk, args(args))]
    pub chunk: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkArgs {
    pub offset: u64,
    pub size: usize,
}

pub fn open_data_chunk(file_path: &str, offset: u64, size: usize) -> BinResult<DataChunk> {
    let file = File::open(file_path)?;
    let mut fits_reader = BufReader::new(file);

    let args = ChunkArgs { offset, size };
    let data_chunk = DataChunk::read_options(&mut fits_reader, binrw::Endian::Big, (args,))?; // read_options expects ags as a single length tuple

    Ok(data_chunk)
}

