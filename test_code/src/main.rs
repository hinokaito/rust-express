struct Matryoshka {
    size: u32,
    next: Option<Box<Matryoshka>>
}

fn main() {
    let m = Matryoshka {
        size: 3,
        next: Some(Box::new(Matryoshka {
            size: 2,
            next: Some(Box::new(Matryoshka {
                size: 1,
                next: None
            }))
        }))
    };
}