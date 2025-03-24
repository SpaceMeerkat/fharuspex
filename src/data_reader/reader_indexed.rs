use binrw::{BinRead, BinResult};
use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::ops::Range;
use crate::header_reader::reader_axes::open_header_axes;

#[binrw::parser(reader, endian)]
pub fn parse_chunk(args: ChunkArgs) -> BinResult<Vec<f64>> { // numpy array is using float64-bit
    let mut chunk_vector = Vec::new();

    let row_stride = args.col_size as usize * 8; // Bytes per row
    let col_stride = 8; // Each f64 value is 8 bytes

    for row in args.row_range.clone() {
        let row_offset = (row * row_stride) as u64;

        for col in args.col_range.clone() {
            let col_offset = (col * col_stride) as u64;
            reader.seek(SeekFrom::Start(args.header_offset + row_offset + col_offset))?;

            let value = f64::read_options(reader, endian, ())?;
            chunk_vector.push(value);
        }
    }

    Ok(chunk_vector)
}

#[derive(BinRead, Debug)]
#[br(big, import(args: ChunkArgs))] // Importing the offset and size from ChunkArgs struct
pub struct DataChunk {
    #[br(parse_with = parse_chunk, args(args))]
    pub chunk: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct ChunkArgs {
    // Change this to take an index vector and an optional size limit?
    pub header_offset: u64,
    pub row_range: Range<usize>,
    pub col_range: Range<usize>,
    pub col_size: u16
}

pub fn open_data_chunk_indexed(file_path: &str, header_offset: u64, row_range: Range<usize>, col_range: Range<usize>) -> BinResult<DataChunk> {

    let file = File::open(file_path)?;

    let headers_binrw = open_header_axes(file_path).unwrap();  // Assuming this returns a FitsHeader
    let header_cards = &headers_binrw.cards;  // Access the cards field

    let first_axis: u16 = header_cards[0].1.parse().expect("Failed to parse first value");
    let col_size: u16 = header_cards[1].1.parse().expect("Failed to parse second value");

    if row_range.end > first_axis.into() || col_range.end > col_size.into()  {
        panic!("Indexing range out of bounds. Row range: {:?}, Col range: {:?}", row_range, col_range);
    }

    // Setup to take a rows:cols range and use that to stride over the bytes and keep the necessary ones

    let mut fits_reader = BufReader::new(file);

    let args = ChunkArgs { header_offset, row_range, col_range, col_size };
    let data_chunk = DataChunk::read_options(&mut fits_reader, binrw::Endian::Big, (args,))?; // read_options expects ags as a single length tuple

    Ok(data_chunk)
} 


