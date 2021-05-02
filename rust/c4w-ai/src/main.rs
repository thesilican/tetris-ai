fn main() {
    let mut set = std::collections::HashSet::new();
    set.insert((0, 1, 0));
    set.insert((0, 1, 0));
    for tup in set {
        println!("{:?}", tup);
    }
}
