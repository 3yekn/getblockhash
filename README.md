# Getting Started
Use a single parameter for the block height.
```bash
wget https://storage.googleapis.com/psl-careers/blk00000.dat
cargo run -- 5
```

# Notes
### Caching
I implemented a version that read and parsed all blocks at the beginning but scrapped that due to the simpler stateless version. The code that creates the cache I left in the unused `util.rs` file.

### Not Implemented
- Error handling
- Logging
- Web Service

