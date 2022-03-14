use basic_types::{Child, Parent, Person, Person2};

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

    // slice
    let v: Vec<f64> = vec![0.0, -0.707, 1.0, 0.707]; // 長さは指定しない
    let a: [f64; 4] = [0.0, -0.707, 1.0, 0.707]; // [T; usize]

    let sv: &[f64] = &v; // ヒープに確保され、所有権のない参照を得る
    let sa: &[f64] = &a; // 参照のみ得ることができ、所有権はない
    println!("sa: {:?}, sv: {:?}", sa, sv);

    let assert = |n: &[f64], v: &[f64]| {
        for (i, element) in n.iter().enumerate() {
            assert_eq!(element, v.get(i).unwrap())
        }
    };
    assert(sv, sa);
    assert(&sv[0..2], &sa[0..2]);
    assert(&sv[2..], &sa[2..]);
    assert(&sv[1..3], &sa[1..3]);

    // 文字列リテラルについて

    // ダブルクオートはエスケープ`\`が必要
    let speech = "\"Ouch!\" said the well.\n";
    print!("{}", speech);

    // 改行と行頭のスペースもそのまま表示される
    let speech = "In the room the women come and, go
    Singing of Mount Abora.";
    print!("{}\n", speech);

    // 行末を`\`で終了すると、行頭のスペースは削除される
    let speech = "It was bright, cold day in April, and\
    there were four of us-\
    more or less.";
    print!("{}\n", speech);

    // 生文字列では、エスケープが不要
    let raw_string = r"C:\Program Files\Gorillas";
    print!("{}", raw_string);

    println!(
        r###"
    This raw string started with 'r###"'.
    Therefore it dose not end untill we reach a quote mark ('"')
    followed immediately by three pound sings ('###'): \n
    "###
    );

    // バイト文字列
    let method = b"GET"; // u8のスライスが得られる
    assert_eq!(&[b'G', b'E', b'T'], method);

    let method = br"POST"; // brで生バイト文字列になる
    assert_eq!(&[b'P', b'O', b'S', b'T'], method);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..]; // oodles
    let poodles = "ಠ_ಠ";
    println!("{}, {}", oodles, poodles);

    assert_eq!(7, "ಠ_ಠ".len());
    assert_eq!(3, "ಠ_ಠ".chars().count());

    // &strは変更できない

    // .to_stringメソッドで&str -> Stringに変換する。文字列をコピーし所有権を得る
    let error_message = "too many pets".to_string();
    assert_eq!(String::from("too many pets"), error_message);
    // to_ownedでも同じようなことができる。ただし他の使い方もある
    let error_message = "too many pets".to_owned();
    assert_eq!(String::from("too many pets"), error_message);

    assert_eq!(format!("{}° {:02}′ {:02}", 24, 5, 23), "24° 05′ 23");

    let bits = vec!["veni", "vidi", "vici"];

    assert_eq!("venividivici", bits.concat());
    assert_eq!("veni, vidi, vici", bits.join(", "));

    // 文字列も == と !=をサポートする
    assert_eq!("ONE".to_lowercase(), "one");
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "x"), "x_x");

    assert_eq!("    clean\n".trim(), "clean");
    assert_eq!("    clean\n".trim_end(), "    clean");
    assert_eq!("    clean\n".trim_start(), "clean\n");

    for word in "veni, vidi, vici".split(",") {
        assert!(word.trim().starts_with("v"));
    }

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];

    // Vec<T>はCopyトレイトを実装していないのでここで値が移動している
    let t = s; // `s`は未初期化状態となり、この後使用できなくなる
    assert_eq!(
        vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()],
        t
    );
    // ここで移動されている値を使っているのでコンパイルエラーになる
    // let u = s;

    // 参照する場合は使える
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = &s;
    assert_eq!(
        &vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()],
        t
    );
    let u = &s;
    assert_eq!(
        &vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()],
        u
    );

    // もしくは、deep copyする
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
    assert_eq!(t, u);

    // 移動を伴う他の操作

    #[allow(unused_assignments)]
    let mut s = "Godiva".to_string();
    s = "Gana".to_string(); // 値"Godiva"がDropされる
    assert_eq!(String::from("Gana"), s);

    let mut s = "Godiva".to_string();
    let t = s; // tに値の所有権が移り、sは未初期化の状態になる（？） -> GodivaはDropされない
    s = "Gana".to_string(); // sに新しい値が書き込まれる
    assert_eq!(String::from("Godiva"), t); // sから所有権を移された値
    assert_eq!(String::from("Gana"), s); // 新しく書き込まれた値

    // 以下のコードでは、初期化や代入以外にも、いくつかの場所で移動が行われている
    let mut composers = Vec::new(); // [move.1] Vec::new()関数からの値の返却
                                    // [move.2] 新しい値（Person構造体）の作成
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    }); // push関数への値渡し
        // Person構造体へのポインタではなく、構造体自体がベクタのpushメソッドに渡される

    // 注意点
    // 1. 移動されるのは値そのものだけであって、それが保有するヒープ上のストレージは移動されない
    //    ベクタや文字列の場合、値そのものは3ワードのヘッダ部分だけである
    // 2. Rustのコンパイラコードの生成はこれらの移動を「見透かす」ことが得意。
    //    -> コンパイラの最適化によって機械語レベルでは移動する場所に始めからオブジェクトを作ってしまうことも多い

    let f = |i: &mut usize| {
        if *i < 10usize {
            *i += 1;
            return true;
        }
        return false;
    };

    let mut i = 0usize;
    while f(&mut i) {}
    assert_eq!(10usize, i);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let third = v[2]; // インデックスで値を引き抜くことができない -> コンパイラに借用（参照）を検討しろと言われる

    // 以下のいずれかの方法でなら値を取り出せる
    let fifth = v.pop().expect("vector is empty!");
    assert_eq!("105", fifth);

    // ベクタの指定したインデックスの場所から値を取り出し、最後の要素を代わりにそこに入れる
    assert_eq!(vec!["101", "102", "103", "104"], v);
    let second = v.swap_remove(1);
    assert_eq!("102", second);
    assert_eq!(vec!["101", "104", "103"], v); // 102がなくなって代わりに104が末尾から移動された！

    // 値を取り出し、任意の値に入れ替える
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!("103", third);
    assert_eq!(vec!["101", "104", "substitute"], v);

    // `for ... in v`のようにしてループにベクタを直接渡すと、ベクタはvから移動される
    let v = vec!["自由".to_string(), "平等".to_string(), "博愛".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let mut composers = Vec::new();
    composers.push(Person2 {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });

    // 以下のようにはできない
    // let first_name = composers[0].name;

    let first_name = std::mem::replace(&mut composers[0].name, None).unwrap();
    assert_eq!("Palestrina".to_string(), first_name);
    assert_eq!(None, composers[0].name);

    // 上のコードは以下のように書き換えることができる
    let mut composers = Vec::new();
    composers.push(Person2 {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });

    // 値を返しながらNoneにReplaceしてくれる
    let first_name = composers[0].name.take().unwrap();
    assert_eq!("Palestrina".to_string(), first_name);
    assert_eq!(None, composers[0].name);

    // コピー型: 移動の例外について

    let string1 = "some string".to_string();
    let string2 = string1;
    assert_eq!("some string".to_string(), string2);
    // println!("{}, {}", string1, string2); // 移動されているためstring1は使えない

    let num1: i32 = 36;
    let num2 = num1;
    println!("{}, {}", num1, num2); // 独立した値のコピーとして確保されるのでnum1がそのまま使える

    // stringの代入 -> 移動, i32の代入 -> Copy

    let p = Parent {
        child: Child { id: 32 },
    };
    let mut p2 = p;
    p2.child.id = 33; // 書き換えてもpには影響がない
    println!("{:?}, {:?}", p, p2); // Copyができるので、pもp2も使用可能

    use std::rc::Rc;
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
    println!("{}, {}, {}", s, t, u);

    // 全ての変数に対して、Stringのメソッドを直接実行することができる
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // s.push_str(" noodles"); // Rcで確保した値は不変となる

    {
        use basic_types::reference::{show, sort_works, Anime, Table, Point};

        let mut table = Table::new();

        table.insert(
            "Gesualdo".to_string(),
            vec![
                "many maidrigals".to_string(),
                "Tenebrare Responsorina".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The musicians".to_string(),
                "The Calling of St. Matther".to_string(),
            ],
        );
        table.insert("Cellini".to_string(), vec![]);

        show(&table);
        sort_works(&mut table);
        show(&table); // 共有参照を用いると、参照先で読むことができるし、所有権が移動しないので再度使える

        let x = 10;
        let r = &x; // &xはxへの共有参照
        assert_eq!(10, *r); // 明示的に`r`の参照を解決

        let mut y = 32;
        let m = &mut y; // 可変参照を作る
        *m += 32; // 参照を解決した先にある値を直接書き換え
        assert_eq!(64, *m); // 32 + 32で64になっているはず

        let aria = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &aria;
        assert_eq!("Aria: The Animation", anime_ref.name); // 下のコードと同じ意味だが、`.`演算子によって参照解決が暗黙的に行われる
        assert_eq!("Aria: The Animation", (*anime_ref).name);

        let mut v = vec![1973, 1968];
        v.sort(); // 暗黙にv屁の可変参照を借用しているので下記のコードと等価である
        (&mut v).sort();

        let x = 10;
        let y = 20;
        let mut r = &x;
        let b = false;

        // bがtrueになるとyの値で書き換えられる
        if b {
            r = &y;
        }
        assert_eq!(*r, 10);

        let point = Point::new(1000, 729);
        let r = &point;
        let rr = &r;
        let rrr = &rr;
        let expect = Point {x: 1000, y: 729};
        assert_eq!(expect, *r);

        assert_eq!(expect, *(*rr));
        assert_eq!(expect, **rr);
        // `.`演算子により、暗黙的にネストした参照を取り払って目的の値を取り出してくれる
        assert_eq!(expect.x, rr.x);
        assert_eq!(expect.y, rr.y);

        assert_eq!(expect, *(*((*rrr))));
        assert_eq!(expect, ***rrr);
        assert_eq!(expect.x, rrr.x);
        assert_eq!(expect.y, rrr.y);
    }
}
