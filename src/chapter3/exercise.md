# 演習

## おみくじ
おみくじを作ってください。

具体的な要件は定義しません。
main関数と出力のみ貼るので合わせてください。

```rust
fn main() {
    let lucky_number = get_random_number();
    println!("{}", lucky_number); // 1~10

    let result = omikuji(lucky_number);
    println!("{}です", result);
}
```

```bash
> cargo run
10
大吉です
```

## 解答例
<details>
<summary>見る</summary>

```rust
use rand::*;

fn main() {
    let lucky_number = random_range(1..=10);

    println!("{}", lucky_number);

    if lucky_number > 7 {
        println!("大吉です");
    }
}
```