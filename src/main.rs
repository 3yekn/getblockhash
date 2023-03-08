use anyhow::Result;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
const BLOCK_FILE: &str = "blk00000.dat";

#[derive(Copy, Clone)]
struct BlockHeader {
    version: u32,
    previous_hash: [u8; 32],
    merkle_root: [u8; 32],
    time: u32,
    n_bits: u32,
    nonce: u32,
}

fn arr_to_hex_swapped(data: &[u8]) -> String {
    data.iter()
        .rev()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

impl fmt::Debug for BlockHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockHeader")
            .field("version", &self.version)
            .field("previous_hash", &arr_to_hex_swapped(&self.previous_hash))
            .field("merkle_root", &arr_to_hex_swapped(&self.merkle_root))
            .field("time", &self.time)
            .field("n_bits", &self.n_bits)
            .field("nonce", &self.nonce)
            .finish()
    }
}

fn seek_to_next_block(mut file: File) -> File {
    
    // assert we are the beginning of a block via magic bytes
    let mut magic_bytes: [u8; 4] = [0u8; 4];
    file.read(&mut magic_bytes).unwrap();
    assert_eq!(&magic_bytes, &MAGIC_BYTES);

    let mut block_size_be: [u8; 4] = [0u8; 4];
    file.read(&mut block_size_be).unwrap();

    let block_size_bytes =
        u32::from_str_radix(arr_to_hex_swapped(&block_size_be).as_str(), 16).unwrap();
    let skip_length = block_size_bytes; 
    file.seek(SeekFrom::Current(skip_length.into())).unwrap();

    file
}

fn get_next_header(mut file: &File) -> Result<BlockHeader> {
    let mut magic_bytes = [0u8; 4];
    file.read_exact(&mut magic_bytes)?;
    assert_eq!(&magic_bytes, &MAGIC_BYTES);

    let mut block_size_be = [0u8; 4];
    file.read_exact(&mut block_size_be)?;

    let mut version_bytes: [u8; 4] = [0u8; 4];
    file.read_exact(&mut version_bytes)?;
    let version = u32::from_le_bytes(version_bytes);

    let mut previous_hash: [u8; 32] = [0u8; 32];
    file.read_exact(&mut previous_hash)?;

    let mut merkle_root: [u8; 32] = [0u8; 32];
    file.read_exact(&mut merkle_root)?;

    let mut time_bytes: [u8; 4] = [0u8; 4];
    file.read_exact(&mut time_bytes)?;
    let time = u32::from_le_bytes(time_bytes);

    let mut num_bits_bytes: [u8; 4] = [0u8; 4];
    file.read_exact(&mut num_bits_bytes)?;
    let n_bits = u32::from_le_bytes(num_bits_bytes);

    let mut nonce_bytes: [u8; 4] = [0u8; 4];
    file.read_exact(&mut nonce_bytes)?;
    let nonce = u32::from_le_bytes(nonce_bytes);

    Ok(BlockHeader {
            version,
            previous_hash,
            merkle_root,
            time,
            n_bits,
            nonce,
        })
}

/// Use load if it is desirable to cache the blocks
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let block_height = &args[1].parse::<u32>().unwrap();

    let mut file = File::open(BLOCK_FILE).unwrap();

    for _ in 0..*block_height {
        file = seek_to_next_block(file);
    }

    let next_header = get_next_header(&file);

    println!(
        "Block: {} has hash: {}",
        block_height,
        arr_to_hex_swapped(&next_header.unwrap().previous_hash)
    );
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_block_hash_3() {
        let block_height = 3;
        let mut file = File::open(BLOCK_FILE).unwrap();
        for _ in 0..block_height {
            file = seek_to_next_block(file);
        }

        let next_header = get_next_header(&file);
        assert_eq!(
            arr_to_hex_swapped(&next_header.unwrap().previous_hash),
            "000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd".to_string()
        );
    }

    #[test]
    fn test_block_hash_222() {
        let block_height = 222;
        let mut file = File::open(BLOCK_FILE).unwrap();
        for _ in 0..block_height {
            file = seek_to_next_block(file);
        }

        let next_header = get_next_header(&file);
        assert_eq!(
            arr_to_hex_swapped(&next_header.unwrap().previous_hash),
            "0000000066356691a4353dd8bdc2c60da20d68ad34fff93d8839a133b2a6d42a".to_string()
        );
    }
}
