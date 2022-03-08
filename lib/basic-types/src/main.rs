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
    assert_eq!(2_u8.pow(8), 255_u8);
    assert_eq!((-4_i32).abs(), 4); // 絶対値
    assert_eq!(0b101101_u8.count_ones(), 4); // ビットカウント

    // 以下は曖昧な数値であるとコンパイラに判断されてコンパイルできない
    // assert_eq!((-4).abs(), 4);
    // 標準ライブラリには、それぞれの整数型に対するメソッドが定義されているが、
    // どの整数型なのかコンパイラが正確に把握できないため

    // 以下のように具体的な型を明示すれば良い
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(i32::abs(-4), 4);
}
