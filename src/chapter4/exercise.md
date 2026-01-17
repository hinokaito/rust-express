# 演習

## コード修正
以下のコードは所有権と借用に関するバグがあるので修正してください。
```rust
fn main() {
    let mut s = String::from("hello");

    print_length(s);

    println!("after print_length: {}", s);

    let r1 = &s;
    let r2 = &mut s;

    println!("{} {}", r1, r2);

    let s2 = add_world(s);

    println!("s: {}", s);
    println!("s2: {}", s2);
}

fn print_length(s: String) {
    println!("length: {}", s.len());
}

fn add_world(s: String) -> String {
    s.push_str(" world");
    s
}
```

<details>
<summary>正解のコードを見る</summary>

```rust
fn main() {
    let mut s = String::from("hello");

    print_length(&s);

    println!("after print_length: {}", s);

    {
        let r1 = &s;
        println!("{}", r1);
    }

    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", r2);

    let s2 = add_world(s);

    println!("s2: {}", s2);
}

fn print_length(s: &String) {
    println!("length: {}", s.len());
}

fn add_world(mut s: String) -> String {
    s.push_str(" world");
    s
}
```