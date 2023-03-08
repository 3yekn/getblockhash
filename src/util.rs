
// Uses the block header cursor type
// For use in caching version
struct BlockHeaderCursor {
    block_header: BlockHeader,
    skip: u32, // length of bytes to skip to the next header
}

// This function reads the entire file and caches all block headers in
// the block_header vector
fn read_entire_file() -> Result<()> {

    let mut block_headers: Vec<BlockHeader> = Vec::new();

    let mut file = File::open(BLOCK_FILE).unwrap();
    let file_length = file.stream_len()?;

    let mut seek_position = 0;
    let mut previous_seek_position = 0;
    let mut block_count = 0;

    while seek_position >= previous_seek_position {
        previous_seek_position = seek_position;

        let mut next_header = get_next_header(&file);

        if next_header.is_ok() {
            block_headers.push(next_header.as_ref().unwrap().block_header);
            block_count += 1;
            seek_position = file.seek(SeekFrom::Current(next_header.unwrap().skip.into()))?;

            // println!(
            //     "Adding block {}; length: {}, seek_position: {}, previous_seek_position: {}",
            //     block_count, file_length, seek_position, previous_seek_position
            // );

        } else {
            break;
        }
    }
    println!();
    Ok(())
}
