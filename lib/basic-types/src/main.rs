fn main() {
    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(2525_u16 as i16, 2525_i16); // in range

    assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

    // 変換先の範囲からはみ出す変換は、元の値の2^Nによる余剰となる
    // Nは変換先のビット数である
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    // 標準ライブラリには、整数型に対するメソッドも定義されている
    assert_eq!(2_u16.pow(4), 16); // 指数関数
    assert_eq!((-4_i32).abs(), 4); // 絶対値
    assert_eq!(0b101101_u8.count_ones(), 4); // ビットカウント

    // 以下は曖昧な数値であるとコンパイラに判断されてコンパイルできない
    // assert_eq!((-4).abs(), 4);
    // 標準ライブラリには、それぞれの整数型に対するメソッドが定義されているが、
    // どの整数型なのかコンパイラが正確に把握できないため

    // 以下のように具体的な型を明示すれば良い
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(i32::abs(-4), 4);

    // 以下のコードはデバッグビルドではパニックが発生するが、リリースビルドではラップされオーバーフローが発生しない
    // use std::time::Duration;
    // {
    //     let mut i: u8 = 1;

    //     loop {
    //         i *= 10; // 3回ループですぐにオーバーフローする
    //         // 正し、デバッグビルドの場合だけ
    //         println!("{}", i); // 10, 100, 232, 16, 160, 64, 128, 0, 0, ....
    //         std::thread::sleep(Duration::from_millis(100));
    //     }
    // }

    // 以下のコードではリリースビルドでもpanicが発生する
    // {
    //     let mut i: i32 = 1;
    //     loop {
    //         i = i.checked_mul(10).expect("multiplication overflowed");
    //     }
    // }

    // チェック付き演算は、数学的に正しい値が得られる場合のみその値をSome(v)として返す
    assert_eq!(10_u8.checked_add(20), Some(30)); // 10と20の和はu8で表現できる
    assert_eq!(100_u8.checked_add(200), None); // 100と200の和はu8で表現できない

    // 加算を行いオーバーフローしていたらパニックを発生させる
    // let x = 100_u8;
    // let y = 200_u8;
    // let sum = x.checked_add(y).unwrap();
    // println!("{}", sum);

    assert_eq!((-128_i8).checked_div(-1), None);

    // ビット単位のシフト演算では、シフトする量が値の範囲に収まるようにラップされる
    // 例えば16ビットの型に対して17ビットシフトしようとすると、１ビットシフトされる
    // 0000000000000101 -> 0000000000001010
    assert_eq!(5_i16.wrapping_shl(17), 10);
    // 00000000000000000000000000001000 -> 00000000000000000000000000010000
    assert_eq!(8_i32.wrapping_shl(33), 8_i32.wrapping_shl(1));

    // staturatingはその型で表現できる最も近い値を返す
    assert_eq!(32760_i16.saturating_add(10), 32767); // i16の最大値で返してくれる
    assert_eq!((-32760_i16).saturating_sub(10), -32768); // 最小値でクランプされる
    // クランプとは留め具のこと。その値の範囲までで固定します。ということらしい :thinking_face:
    // 除算・乗算、ビット単位シフトには飽和演算はない

    // overflowingは(result: T, overflowd: bool)という形のタプルを返す
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true)); // overflowが発生し、発生したことが伝えられる

    // overflowing_shl: ビット左シフトとoverflowing_shr: ビット右シフトはパターンから少し逸脱しており、
    // シフト量がその型のビット数以上だった場合にoverflowedがtrueに評価される
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
    // 17ビットシフトはu16には大きすぎる。17 / 16の剰余は1なので、１ビットだけシフトされる(wrapping + overflowed)?
}
