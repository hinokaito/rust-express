# 演習

以下の固定長配列の要素をすべて2倍にし、ベクタに順番通り格納し、出力してください。
```rust
fn main() {
    let x: [i32; 5] = [1, 2, 3, 4, 5];
}
```

<details>
<summary>正解のコードを見る</summary>

```rust
fn main() {
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    // ベクタを可変で初期化
    let mut v = Vec::new();

    // インデックス指定するために0から長さ(len)手前まで
    for i in 0..x.len() {
        v.push(x[i]*2);
    }

    println!("{:?}", v);
}
```