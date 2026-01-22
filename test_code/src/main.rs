const CONDITION: bool = true;
static X: u8 = if CONDITION { 1 } else { 0 };

fn main() {
    println!("{}", X);
}