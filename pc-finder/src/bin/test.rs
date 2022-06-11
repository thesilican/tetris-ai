use common::*;
use pc_finder::*;

fn main() -> GenericResult<()> {
    // let tess = "BhgAAAAACCCAAAIJAAAAHCACgAMEAAACAAAAARwTAAAIIMAbAQQwAAAMAABAwAEUYAADAAAFEMAAAgAO";
    // println!("{}", Tess::base64_deserialize(tess)?);
    // let board = "2wcQAAA=";
    // println!("{}", PcBoard::base64_deserialize(board)?);

    dbg!(std::mem::size_of::<PcTableVal>());

    // let res = fetch_parents(PcBoard::base64_deserialize("AAAAAAAAAAA")?)?;
    // for parent in res {
    //     println!("{}\n", parent);
    // }
    Ok(())
}
