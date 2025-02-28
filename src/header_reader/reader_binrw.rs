use binrw::{binrw, BinRead, BinWrite};
use std::fs::File;
use std::io::{BufReader};

#[binrw]
#[derive(Debug)]
struct FitsHeaderCard {
    #[br(map = |key: [u8; 8]| String::from_utf8_lossy(&key).trim().to_string())]
    #[bw(map = |key: &String| key.as_bytes())]
    key: String,

    _equals: [u8; 1], // Skip '=' character

    #[br(map = |buf: [u8; 70]| String::from_utf8_lossy(&buf).split('/').next().unwrap_or("").trim().to_string())]
    value: String,
}

pub fn open_header_binrw(file_path: &str) -> binrw::BinResult<Vec<FitsHeaderCard>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut headers = Vec::new();

    while let Ok(header_card) = FitsHeaderCard::read(&mut reader) {
        if header_card.key == "END" {
            break;
        }
        headers.push(header_card);
    }

    Ok(headers)
}
