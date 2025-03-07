use binrw::{BinRead, BinResult};
use std::fs::File;
use std::io::{BufReader, SeekFrom};
use crate::header_reader::reader_axes::open_header_axes;

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

pub fn open_data_chunk_indexed(file_path: &str, offset: u64, size: usize) -> BinResult<DataChunk> {
    let file = File::open(file_path)?;

    let headers_binrw = open_header_axes(file_path).unwrap();  // Assuming this returns a FitsHeader
    let header_cards = &headers_binrw.cards;  // Access the cards field

    let first_axis: u8 = header_cards[0].1.parse().expect("Failed to parse first value");
    let second_axis: u8 = header_cards[1].1.parse().expect("Failed to parse second value");

    let strides = vec![first_axis * second_axis, second_axis];
    println!("Strides set to: {:?}", strides);

    // Setup to take a rows:cols range and use that to stride over the bytes and keep the necessary ones

    let mut fits_reader = BufReader::new(file);

    let args = ChunkArgs { offset, size };
    let data_chunk = DataChunk::read_options(&mut fits_reader, binrw::Endian::Big, (args,))?; // read_options expects ags as a single length tuple

    Ok(data_chunk)
}

