use std::fs::File;
use std::io::{BufReader, Read, Result};

// Reads the first 2880 bytes (header).

#[derive(Debug)]
struct FitsHeader {
    key: String,
    value: String,
}

fn open_header(file_path: &str) -> Result<Vec<FitsHeader>> {
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

    let mut headers = Vec::new();
    // Define an empty vector to store the headers on the heap

    for chunk in buffer.chunks(80) { // Each header entry has 80 bytes
        let line = String::from_utf8_lossy(&chunk); // Changes a byte chunk into a string
        if line.starts_with("END") { // If the last header key is END, then break the loop
            break;
        }
        if let Some((key, rest)) = line.split_once("=") {
            let key = key.trim().to_string();
            let value = rest.split('/').next().unwrap_or("").trim().to_string();
            headers.push(
                FitsHeader {key: key, value: value}
            );
        }
    }

    Ok(headers)
}

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
