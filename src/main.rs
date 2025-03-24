use std::io::Result;
use fharuspex::header_reader::{reader::open_header, reader_binrw::open_header_binrw, reader_axes::open_header_axes};
use fharuspex::data_reader::{reader::open_data_chunk, reader_indexed::open_data_chunk_indexed};

fn main() -> Result<()> {

    // Declare the filepath
    let file_path: &str = "datasets/example.fits";
    println!("Filename set to {}.", &file_path);

    // Open the header without using binwr
    let headers = open_header(file_path);
    println!("\nHeader opened without using binrw: \n");
    for header in headers?.iter() {
        println!("{:?}", header);
    }

    // Open the header with binrw
    let headers_binrw = open_header_binrw(file_path).unwrap();  // Assuming this returns a FitsHeader
    let header_cards = &headers_binrw.cards;  // Access the cards field
    // Iterate over the cards and print each key-value pair on a new line
    println!("\n\nHeader opened using binrw: \n");
    for (key, value) in header_cards {
        println!("{} = {}", key, value);
    }

    // Open the header axes with binrw
    let headers_binrw = open_header_axes(file_path).unwrap();  // Assuming this returns a FitsHeader
    let header_cards = &headers_binrw.cards;  // Access the cards field
    // Iterate over the cards and print each key-value pair on a new line
    println!("\n\nHeader axes opened using binrw: \n");
    for (key, value) in header_cards {
        println!("{} = {}", key, value);
    }

    // Open the data chunk with binrw, without contiguous indexing
    let header_offset = 2880;
    let offset = 3 * 8; // User specified 
    let size = 2;
    let bitpix = -64;
    let data_values = open_data_chunk(file_path, offset + header_offset, size);
    println!("Data vector read as: {:?}", data_values);

    // Open a data chunk with binrw using contiguous indexing and row col ranges
    let row_range = 10..13;
    let col_range = 28..30;
    let indexed_data_values = open_data_chunk_indexed(file_path, header_offset, row_range, col_range);
    println!("Data vector using indexing stride: {:?}", indexed_data_values);
    
    Ok(())
}
