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

    {
        use error_handling::results::*;

        let display_weather = |weather: Result<Weather, String>| match weather {
            Ok(weather) => println!("Weather: {:?}", weather),
            Err(message) => println!("Err: {}", message),
        };

        let weather = get_weather(LatLng {
            lat: 10f64,
            lng: 100f64,
        });
        display_weather(weather);
        let weather = get_weather(LatLng {
            lat: 25f64,
            lng: 35f64,
        });
        display_weather(weather);
        let weather = get_weather(LatLng {
            lat: 101f64,
            lng: 10f64,
        });
        display_weather(weather);

        let result = get_weather(LatLng {
            lat: 10f64,
            lng: 100f64,
        });
        // Resultにあるよく使うメソッド
        assert!(result.is_ok()); // Okかどうか
        assert!(!result.is_err()); // Errかどうか

        // ResultがOkであれば、Some(success_value)が帰る
        if let Some(success_value) = result.ok() {
            assert_eq!(Weather::Sunny, success_value);
        } else {
            panic!("この行は呼ばれないはず")
        }

        let result = get_weather(LatLng {
            lat: 101f64,
            lng: 10f64,
        });
        // ResultがErrであればOption<E>が帰る
        if let Some(e) = result.err() {
            assert_eq!(String::from("Invalid location."), e);
        } else {
            panic!("この行は呼ばれないはず")
        }

        let result = get_weather(LatLng {
            lat: 101f64,
            lng: 10f64,
        });
        assert!(result.is_err());
        // unwrap_orはErrだった場合fallbackに指定した値を返す
        assert_eq!(Weather::Sunny, result.unwrap_or(Weather::Sunny));

        // unwrap_or_elseはunwrap_orとほぼ同じだが、クロージャを指定する
        let result = get_weather(LatLng {
            lat: 101f64,
            lng: 10f64,
        });
        assert!(result.is_err());
        assert_eq!(
            Weather::Cloudy,
            result.unwrap_or_else(|_err| Weather::Cloudy)
        );

        // unwrap / expectは成功だった場合のみ値を返し、エラーだった場合はpanicを起こしてしまう
        let _result = get_weather(LatLng {
            lat: 10f64,
            lng: 100f64,
        })
        .unwrap();
        // expectの場合はmessageを指定することができる
        let _result = get_weather(LatLng {
            lat: 10f64,
            lng: 100f64,
        })
        .expect("expected!");

        // 以下のメソッドは、Resultの値を`借用`できる
        let result = get_weather(LatLng {
            lat: 25f64,
            lng: 35f64,
        });
        // as_refはResult<T, E>をResult<&T, &E>の共用参照に変換する
        assert_eq!(Ok(&Weather::Cloudy), result.as_ref());
        let mut result = get_weather(LatLng {
            lat: 101f64,
            lng: 10f64,
        });
        // as_mutは可変参照に変換する
        assert_eq!(Err(&mut String::from("Invalid location.")), result.as_mut());

        // resultを壊さずに取り出す必要が有る場合以下のようにする
        let result = get_weather(LatLng {
            lat: 25f64,
            lng: 35f64,
        });
        // Option<&T>が取得できるが、借用されるだけでresultは移動されない
        let v = result.as_ref().ok();
        assert_eq!(Some(&Weather::Cloudy), v);
        assert_eq!(Some(Weather::Cloudy), result.ok()); // ここで初めて移動される

        {
            // Result型のエイリアスを作ることができる
            type Result<Weather> = std::result::Result<Weather, String>;
            fn try_get_weather(location: LatLng) -> Result<Weather> {
                // 返り値がResult<T, E>の関数内で、Result<?, E>を返す関数の場合、
                // `?`をつけることでErr(E)が返った場合自動的にreturnしてくれる
                let weather = get_weather(location)?;

                Ok(match weather {
                    Weather::Cloudy => Weather::Sunny,
                    Weather::Sunny => Weather::Cloudy,
                })
            }

            assert_eq!(
                Ok(Weather::Sunny),
                try_get_weather(LatLng {
                    lat: 25f64,
                    lng: 35f64,
                })
            );
        }

        {
            // `?`で返却できるのはResult<T, E>のEに変換可能なもののみ
            // エラー型の変換処理を実装するか、全てのエラーを表すシグネチャを作成する
            // GenericError::fromで既存のエラー型を総称できる
            type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
            use std::io;
            let io_error = io::Error::new(io::ErrorKind::Other, "timed out");

            // fromメソッドでエラーに変換できる
            let g_error = GenericError::from(io_error);
            println!("{:?}", &g_error);

            // 特定のエラーだった場合のみリトライを実施したい時などは、error.downcast_ref::<ErrorType>()を用いる
            let r: Result<(), GenericError> = Err(g_error);
            match r {
                Ok(()) => panic!("Unexpected operation"),
                Err(err) => {
                    // 総称型の中からio:Errorだった場合のみ取り出す
                    if let Some(msg) = err.downcast_ref::<io::Error>() {
                        println!("{}", msg);
                    } else {
                        panic!("Unexpected operation");
                    }
                }
            }
        }

        {
            // 起こるはずのないエラーの場合は、unwrapしてしまう
            let digits = "bleen";
            let num = digits.parse::<u64>();
            println!("{:?}", num);

            let num = "3.14".parse::<f64>().unwrap();
            assert_eq!(3.14f64, num);

            use error_handling::errors::*;
            let c = CustomError {
                message: "Error".to_string(),
                line: 10,
                column: 100,
            };
            println!("{:?}", c);
        }
    }
}
