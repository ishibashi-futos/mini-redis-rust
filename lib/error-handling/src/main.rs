fn main() {
    {
        // パニックが起きた時、Rustはスタックを巻き戻すかプロセスをabortするかのどちらかを行う
        // Rustのデフォルト動作は巻き戻す方

        // 以下の行はゼロ除算が発生してpanicを起こす
        use error_handling::*;
        // let _s = rewind::pirate_share(10, 0);
        // メインスレッド内でpanicが発生すると、プロセスが終了する
        assert_eq!((5, 2), rewind::pirate_share(10, 2));

        // threadの中でpanicが発生した場合、作られた変数などは逆順にドロップされる
        use std::thread;
        let handler = thread::spawn(|| {
            let _s = rewind::pirate_share(10, 0);
        });
        // スレッド内で発生したpanicは、呼び出し元に返される
        let err = handler.join().err();

        // エラーが発生しているので`Some(Any { .. })`が出る
        println!("OK: {:?}", err);
    }
}
