fn print_owned(value: Box<dyn ToString>) {
    println!("{}", value.to_string());
}

fn main() {
    let val: Box<dyn ToString> = Box::new(10);
    print_owned(val);
    print_owned(val);
}