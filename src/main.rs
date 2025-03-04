use std::io::Result;
use fharuspex::header_reader::reader::open_header;
use fharuspex::header_reader::reader_binrw::open_header_binrw;

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
    
    Ok(())
}
