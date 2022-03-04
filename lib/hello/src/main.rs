use std::str::FromStr;
use std::env;

use hello::gcd;

fn main() {
    let mut numbers = Vec::new();

    // 1つ目の引数にはバイナリのパスが格納されるのでスキップ
    for arg in env::args().skip(1) {
        // u64に実装されたfrom_str関数を使用してinputがu64にパース可能かチェックしながらパース
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        // 失敗するとexpectのメッセージを吐き出しながらpanicする
        // panicで終了した場合、終了ステータスコードは`101`となる
    }

    // inputの長さが足りていない場合
    if numbers.len() == 0 {
        // エラー出力を吐き出す
        eprintln!("Usage: gcd NUMBER ...");
        // processを0以外で終了させる
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // １つ目以降の要素を使用して、全ての値に対する最大公約数を求める
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
