use std::io::Result;
use fharuspex::header_reader::reader::open_header;

fn main() -> Result<()> {
    // Declare the filepath
    let file_path: &str = "datasets/example.fits";
    println!("Filename set to {}.", &file_path);
    // Define the file to open
    let headers = open_header(file_path);
    for header in headers?.iter() {
        println!("{:?}", header);
    }
    
    Ok(())
}
