# 演習
1. `Shape`列挙型を作ってください。中身は`Triangle`(△)と`Square`(□)です
2. それぞれの列挙型にi32のbottomとheightを持たせてください。
3. `Shape`に面積を求めて返す、`area`関数を実装してください

main関数は以下の通りです。

```rust
fn main() {
    let t = Shape::Triangle {
        bottom: 10,
        height: 10
    };

    let s = Shape::Square {
        bottom: 10,
        height: 10
    };

    println!("{}", t.area());
    println!("{}", s.area());
}
```

<details>
<summary>正解のコードを見る</summary>

```rust
enum Shape {
    Triangle { bottom: u32, height: u32 },
    Square { bottom: u32, height: u32 }
}

impl Shape {
    fn area(&self) -> u32 {
        match self {
            Self::Triangle { bottom, height } => {
                *bottom * *height / 2
            },
            Self::Square { bottom, height } => {
                *bottom * *height
            }, 
        }
    }
}

fn main() {
    let t = Shape::Triangle {
        bottom: 10,
        height: 10
    };

    let s = Shape::Square {
        bottom: 10,
        height: 10
    };

    println!("{}", t.area());
    println!("{}", s.area());
}
```