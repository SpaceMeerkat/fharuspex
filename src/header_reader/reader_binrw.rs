use binrw::{BinRead, BinResult, Endian};
use std::fs::File;
use std::io::{BufReader};

#[binrw::parser(reader, endian)] // Headers are ASCII, so endian is needed
pub fn parse_cards() -> BinResult<Vec<(String, String)>> {
    let mut headers = Vec::new(); // Fill vector with header key:value pairs
    // let endian = Endian::Big; // This feels icky

    loop {
        let card_bytes = match <[u8; 80]>::read_options(reader, endian, ()) { // Read 80 bytes
            Ok(card_bytes) => card_bytes,
            Err(err) => {
                eprintln!("Error reading 80 bytes into buffer: {}", err);
                panic!("Terminating due to buffer read.");
            }
        };

        let card = String::from_utf8_lossy(&card_bytes).trim().to_string();

        if card.starts_with("END") {
            break;
        }

        if let Some((key, value)) = card.split_once("=") {
            headers.push((key.trim().to_string(), value.split('/').next().unwrap_or("").trim().to_string()));
        }
    }

    Ok(headers)
}

#[derive(BinRead)]
#[derive(Debug)]
#[br(big)]
pub struct FitsHeader {
    #[br(parse_with = parse_cards)]
    pub cards: Vec<(String, String)>,
}

pub fn open_header_binrw(file_path: &str) -> BinResult<FitsHeader> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file `{}`: {}", file_path, err);
            panic!("Terminating due to file read error.");
        }
    };

    let mut fits_reader = BufReader::new(file); // Create a new buffer from file
    
    // Read the header, passing Endian::Big for endian and an empty tuple for options
    let headers = match FitsHeader::read(&mut fits_reader) {
        Ok(headers) => headers,
        Err(err) => {
            eprintln!("Error creating the header: {}", err);
            panic!("Terminating due to header writing error");
        }
    };

    Ok(headers)
}
