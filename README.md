# LARGE_NUMBER

## Overview
Sbiis Saibianが考案した巨大数を表記する方法であるハイパーE表記を一部採用し、
それらを使った演算ができるLibです。

## Requirement
動作確認した環境
- windows11
- x86-64

## Reference
wikipediaのハイパーE表記に関する記述
(https://ja.wikipedia.org/wiki/%E3%83%8F%E3%82%A4%E3%83%91%E3%83%BCE%E8%A1%A8%E8%A8%98)

## Usage
足し算をするサンプルコードです。
```rust
use LargeNum;

fn foo() {
    let x = LargeNum::new(1729);
    let y = LargeNum::new(1111);

    println!("{}", x + y) // 2840
}
```

## Features
現在対応している演算
| 対応済みかどうか | 名前 | プログラム |
| :-- | :---------- | :-------- |
| [x] | add         | a + b     |
| [x] | multiple    | a * b     |
| [x] | power       | a.pow(b)  |
| [ ] | subtraction | a - b     |
| [ ] | division    | a / b     |

## Warning
このライブラリは未完成のため大幅な互換性のない変更される場合があります。

## Licence
MIT (https://github.com/lightstarp/large_number/blob/master/LICENSE)