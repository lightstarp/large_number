# LARGE_NUMBER

## Overview
Sbiis Saibianが考案した巨大数を表記する方法であるハイパーE表記を一部採用し、
それらを使った演算ができるLibです。

## Requirement
動作確認した環境
- windows11
- x86-64

## Reference
wikipediaで記載されている[ハイパーE表記](https://ja.wikipedia.org/wiki/%E3%83%8F%E3%82%A4%E3%83%91%E3%83%BCE%E8%A1%A8%E8%A8%98)

## Usage
演算をするサンプルコードです。
```rust
use large_number::Lnum;

fn add() {
    let x = Lnum::new(1729);
    let y = Lnum::new(1111);

    println!("{}", x + y) // 2840.00
}

fn multiple() {
    let x = Lnum::new(1729);
    let y = Lnum::new(1111);

    println!("{}", x + y) // E6.28
}
```

## Features
Lnumの最小値と最大値
| Min | Max         |
| :-- | :---------- |
| 0   | E99.9#32767 |

現在対応している演算や今後実装予定の演算
| 対応済みかどうか | 名前 | コード |
| :-- | :---------- | :-------- |
| Yes | add         | a + b     |
| Yes | multiple    | a * b     |
| Yes | power       | a.pow(b)  |
| No  | subtraction | a - b     |
| No  | division    | a / b     |
| Yes | ord(left)   | a > b     |
| Yes | ord(right)  | a < b     |
| Yes | eq          | a == b    |

## Warning
このライブラリは現在未完成のため互換性のない変更される可能性があります。（というか、変更します）

## Licence
Licence [MIT](https://github.com/lightstarp/large_number/blob/master/LICENSE.md)