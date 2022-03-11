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

    // 浮動小数点数
    let f = 31415.926e-4f64;

    println!("f: {}", f); // `f: 3.1415926`

    assert_eq!(5f32.sqrt() * &5f32.sqrt(), 5.); // ちょうど5.0になるはず
    assert_eq!((-1.01f64).floor(), -2.0);

    // 浮動小数点数にも数学計算用のメソッドが定義される
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    // 文字
    assert_eq!('*', '\x2A'); // '*'のコードポイントはUnicodeでは2Aなので、等価である
    assert_eq!('*' as i32, 42); // 10進数表現の場合42になるので等価
    assert_eq!('ಠ' as u16, 0xca0); // U+0CA0が符号付き8ビットに丸められた
    assert_eq!('ಠ' as i8, -0x60);
    println!("{}", '\u{CA0}');

    assert_eq!('*'.is_alphabetic(), false);
    assert!('a'.is_alphabetic());
    assert_eq!('8'.to_digit(10), Some(8)); // char to num
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2')); // num to char

    // ポインタ型について
    // Javaで以下のようなTuple型を用いた場合、基本的にあるオブジェクトが持つ値はオブジェクトへの参照となるが、
    // Rustの場合ローカル変数に４つの隣接した値として確保され、ヒープには何も確保されない
    let p = ((0, 0), (1440, 900));
    println!("{:?}", p);

    let s = String::from("Hello");
    println!("&s: {}", &s); // &sはxへの`参照を借用する`
    let s2 = &s;
    println!("*s2: {}", *s2); // *s2は、s2が指す値を取得する
    let mut s = String::from("Hello");
    let s2 = &mut s;
    (*s2).push_str(", World");
    println!("s2: {}", s);

    // ヒープに値を確保する方法
    {
        let t = (12, "eggs");
        let b = Box::new(t);
        println!("b: {:?}", b);
    }
    // bがスコープを外れると、自動的にヒープから解放される

    // 配列
    let buffer = [0u8; 1024];
    // [V; N] -> V: 初期値, N: 要素数
    println!("buffer: {}", buffer.len());
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!([1, 2, 3, 4, 5], chaos);

    // Vector
    // Vectorは、ヒープに確保されるサイズを変更できる配列
    let primes = vec![2, 3, 5, 7]; // vecマクロで初期化する
    assert_eq!(primes.iter().product::<i32>(), 210);

    // 配列と同じように初期値と数を指定してVectorを作ることもできる
    let new_pixex_buffer = |rows: usize, cols: usize| vec![0u8; rows * cols];
    let buffer = new_pixex_buffer(1024, 1024);
    println!("buffer: {}", buffer.len());

    // vec!マクロは新しいからのVectorをnewで作ってから要素を追加するのと等価（同じコードになる）
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(vec!["step", "on", "no", "pets"], pal);

    // イテレータが生成する値から配列を作ることもできる
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(vec![0, 1, 2, 3, 4], v);

    let mut palindrome = vec!["a man", "a plan", "a cancel", "panama"];
    palindrome.reverse();
    assert_eq!(vec!["panama", "a cancel", "a plan", "a man"], palindrome);

    // capacityが決まっていないベクターに値を追加しようとすると、
    // より大きなバッファが確保され、現在の要素がそちらにコピーされたのち、
    // ベクターのポインタと内容が更新され新しいバッファを参照するようになる、古いバッファは解放される -> Very heavy

    // あらかじめ大きさがわかっている場合は、with_capacityを使う
    // with_capacityを用いると、あらかじめ必要なバッファを確保してベクタを作ってくれる
    let mut v = Vec::with_capacity(2);
    assert_eq!(0, v.len());
    assert_eq!(2, v.capacity());
    v.push("Hell");
    v.push(",");
    assert_eq!(2, v.len());
    assert_eq!(2, v.capacity());
    v.push("World");
    assert_eq!(3, v.len());
    assert!(3 <= v.capacity()); // ちょうど4ではない可能性はあるが、少なくとも3以上になる

    let mut v = vec![10, 20, 30, 40, 50];
    // insert/removeは要素を全てシフトすることになるので、ベクタが長い場合は処理に時間がかかる
    v.insert(3, 35);
    assert_eq!(vec![10, 20, 30, 35, 40, 50], v);
    v.remove(1);
    assert_eq!(vec![10, 30, 35, 40, 50], v);

    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(Some("Glass Gem"), v.pop());
    assert_eq!(Some("Snow Puff"), v.pop());
    assert_eq!(None, v.pop());
}
