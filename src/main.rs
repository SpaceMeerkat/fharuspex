use std::fs::File;
use std::io::{self, BufReader, Read};

// Reads the first 2880 bytes (header).

fn main() -> io::Result<()>{
    // Declare the filepath
    let file_path: &str = "datasets/example.fits";
    println!("Filename set to {}.", &file_path);
    // Define the file to open
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: failed top open file `{}`. \nDetails: {}", file_path, err);
            panic!("Terminating due to file open failure");
        }
    };
    // Create a mutable reader for the buffer wrapping the file
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 2880]; // An array of 2880 zero-initialised unsigned 8-bit intigers
    // Here we use an array as it's stack allocated rather than Vec<> which is heap allocated and slower

    match reader.read_exact(&mut buffer) {
        Ok(_) => {
            println!("Successfully read 2880 bytes from the FITS file.");
        },
        Err(err) => {
            eprintln!("Error: Failed to read from file `{}`. \nDetails: {}", file_path, err);
            panic!("Terminating due ot read failure");
        }
    };

    Ok(())

}
