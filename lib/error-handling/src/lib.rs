pub mod rewind {
    /// クルーにシェアする`わけまえ`を計算する
    /// 船長が半分をとり、残りを船員が等分に山分けする。
    /// 余が出たら船に住むオウムがもらう
    pub fn pirate_share(total: u64, crew_size: usize) -> (u64, u64) {
        let half = total / 2;
        (half, half / crew_size as u64)
    }
}
