use std::io::Result;
use fharuspex::header_reader::reader::open_header;

use fharuspex::header_reader::reader_binrw::open_header_binwr;

fn main() -> Result<()> {
    // Declare the filepath
    let file_path: &str = "datasets/example.fits";
    println!("Filename set to {}.", &file_path);
    // Define the file to open
    let headers = open_header(file_path);
    for header in headers?.iter() {
        println!("{:?}", header);
    }

    let headers_binwr = open_header_binwr(file_path);
    for header in headers_binrw {
        println!("{:?}", header);
    }
    
    Ok(())
}
