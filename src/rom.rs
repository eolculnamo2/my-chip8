// will read ROM
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn fetch_rom() -> io::Result<Vec<u8>> {
    // let mut ch_file = File::open("clock.ch8")?;
    let mut ch_file = File::open("ibm.ch8")?;
    let mut ch_buffer: [u8; 1000] = [0; 1000];
    let data_length = ch_file.read(&mut ch_buffer[..])?;
    let trimmed_buffer = Vec::from(&ch_buffer[0..data_length]);
    // trimmed_buffer.iter().for_each(|b| {
    //     if *b != 0 {
    //         println!("{b}")
    //     }
    // });
    Ok(trimmed_buffer)
}
