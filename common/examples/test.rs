use common::misc::ArrDeque;

fn main() {
    let mut arr = ArrDeque::<i32, 5>::new();
    arr.push_back(1);
    arr.push_back(2);
    arr.push_back(3);
    println!("{:?}", &arr);
    arr.push_back(4);
    arr.push_back(5);
    arr.push_back(6);
    println!("{:?}", &arr);
    arr.pop_front();
    let pop = arr.pop_front();
    println!("{:?} {:?}", pop, &arr);
    arr.push_back(6);
    arr.push_back(7);
    println!("{:?}", &arr);
    println!("{:?}", &arr.iter().collect::<Vec<_>>())
}
