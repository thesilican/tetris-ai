use common::*;
use pc_finder::*;

fn main() -> GenericResult<()> {
    // let tess = Tess::base64_deserialize("AAMAAwAAAAAAAAAMAAwAAAAAAAAA8AAAAAAAAAEAAAAAcAAgAAACAAAAAgADAAIAAgMAAAAAABAAcAMCAAAAAAAHAAEEAAAAAAAACAAOBAIAAAAAAMABgAUAAwABgAAAAAAGAA")?;
    // println!("{}", tess);
    let board = PcBoard::base64_deserialize("AAMAAwAAAAA")?;
    println!("{}", board);

    // let res = fetch_parents(PcBoard::base64_deserialize("AAAAAAAAAAA")?)?;
    // for parent in res {
    //     println!("{}\n", parent);
    // }
    Ok(())
}
