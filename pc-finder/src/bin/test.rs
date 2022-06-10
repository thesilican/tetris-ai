fn type_name_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    println!("{}", type_name_of_val(b"hi"));
}
