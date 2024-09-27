use libtetris::*;
use pc_finder::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("data/pc-table.bin").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    PcFinderAi::new(&buf).demo();
}
