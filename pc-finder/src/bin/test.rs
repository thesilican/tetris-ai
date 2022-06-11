use common::*;
use pc_finder::*;

fn main() -> GenericResult<()> {
    let tess = Tess::base64_deserialize("AAMAAwAAAAAAAAAMAAwAAAAAAAAA8AAAAAAAAAEAAAAAcAAgAAACAAAAAgADAAIAAgMAAAAAABAAcAMCAAAAAAAHAAEEAAAAAAAACAAOBAIAAAAAAMABgAUAAwABgAAAAAAGAA")?;
    println!("{}", tess);
    // let board = PcBoard::base64_deserialize("AwABgAAAAAA=")?;
    // println!("{}", board);
    Ok(())
}
