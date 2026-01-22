# 演習

- 出力: `println!("")`
- 変数宣言: `let`
- 条件分岐: `if`
- ループ: `for`

## 1. FizzBuzz
1~100のFiizBuzzを作ってください。
FizzBuzzは値に応じて出力する文字を変えるものです。
- 3で割場合"Fizz"
- 5で割れる場合"Buzz"
- 3でも5でも割れる場合"FizzBuzz"
- それ以外はそのまま数字を

実行例
```bash
1
2
Fizz
4
Buzz
Fizz
# ...
#...
98
Fizz
Buzz
```

## 2. 九九
九九を作ってください

期待する結果
```bash
1 2 3 4 5 6 7 8 9
2 4 6 8 10 12 14 16 18
# ...
# ...
9 18 27 ... 81
```


## 解答例
<details>
<summary>FizzBuzz</summary>

```rust
fn main() {
    let start = 1;
    let end = 100;

    for n in start..=end {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", n);
        }
    }
}
```

</details>

<details>
<summary>九九</summary>


```rust
fn main() {
    for ln in 1..=9 {
        for col in 1..=9 {
            print!("{} ", ln * col);
        }
        println!("");
    }
}
```

</details>