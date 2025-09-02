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

## Warning
このライブラリは未完成のため大幅に変更される場合があります。

## Usage
足し算をするサンプルコードです。
```
use HLarge;

fn foo() {
    let x = HLarge::new(1729);
    let y = HLarge::new(1111);

    println!("{}", x + y) // 2840
}
```

## Licence
MIT (https://github.com/lightstarp/large_number/blob/master/LICENSE)