use libtetris::*;

fn main() {
    println!("{}", std::mem::size_of::<Game>());
    println!("{}", std::mem::size_of::<Child>());
    println!("{}", std::mem::size_of::<Option<Child>>());
}
