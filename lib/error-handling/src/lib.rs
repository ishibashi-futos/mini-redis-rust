pub mod rewind {
    /// クルーにシェアする`わけまえ`を計算する
    /// 船長が半分をとり、残りを船員が等分に山分けする。
    /// 余が出たら船に住むオウムがもらう
    pub fn pirate_share(total: u64, crew_size: usize) -> (u64, u64) {
        let half = total / 2;
        (half, half / crew_size as u64)
    }
}

pub mod results {
    pub struct LatLng {
        pub lat: f64,
        pub lng: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum Weather {
        Sunny,
        Cloudy,
    }

    pub fn get_weather(location: LatLng) -> Result<Weather, String> {

        if location.lat == 10f64 && location.lng == 100f64 {
            return Ok(Weather::Sunny);
        }

        if location.lat == 25f64 && location.lng == 35f64 {
            return Ok(Weather::Cloudy);
        }

        Err("Invalid location.".to_string())
    }
}
