/// count_selected_cities関数のテストを、count_selected_cities_has_monster_attacks2で書こうとすると、
/// fn(&City) -> boolのままではコンパイルできない。Fn/FnOnce/FnMutを使用する
/// 全てのクロージャはそれぞれ固有の型を持っている。
/// クロージャには外側からデータを借り受けたり、移動したりしたデータが含まれる可能性があるためである
///
/// Rustのクロージャは高速に動作するように作られている
/// ほとんどの言語では、クロージャはヒープ上に作成され、動的に実行され、GCで回収されてしまうが、
/// Rustのクロージャはこれらの性能上の欠点が存在しない
/// BoxやVecなどのコンテナに入れない限り、ヒープ上に確保されることはない
/// また、呼び出そうとするクロージャの型をRustのコンパイラが知ることができれば、コードをインライン化できる

pub struct City {
    pub name: String,
    pub monster_attack_risk: f64,
}

pub fn count_selected_cities<F>(cities: &Vec<City>, test: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test(city) {
            count += 1;
        }
    }
    count
}

pub fn dropper() {
    let my_str = "Hello".to_owned();
    let f = || drop(my_str);

    f();
    // f(); // my_strは既に消費されているので、２回呼び出せないのでコンパイルエラーになる
    // Javaとかでやりがちな２回コネクション解放をするみたいなエラーが起きない
}

pub fn do_mutable<F>(mut f: F)
where
    F: FnMut(&mut String),
{
    let mut str = "Hello".to_owned();
    f(&mut str);
}

#[cfg(test)]
mod tests {
    use crate::city::*;
    use expect::expect;

    #[test]
    fn count_selected_cities_has_monster_attacks() {
        let has_monster_attacks = |city: &City| city.monster_attack_risk > 0.0;

        let cities = vec![
            City {
                name: "A".to_owned(),
                monster_attack_risk: 1.0,
            },
            City {
                name: "B".to_owned(),
                monster_attack_risk: 1.1,
            },
            City {
                name: "C".to_owned(),
                monster_attack_risk: 0.0,
            },
        ];

        let n = count_selected_cities(&cities, has_monster_attacks);
        expect(&n).equals(&2);
    }

    #[test]
    fn count_selected_cities_has_monster_attacks2() {
        let cities = vec![
            City {
                name: "A".to_owned(),
                monster_attack_risk: 1.0,
            },
            City {
                name: "B".to_owned(),
                monster_attack_risk: 1.1,
            },
            City {
                name: "C".to_owned(),
                monster_attack_risk: 0.0,
            },
        ];

        let limit = 1.0;
        let n = count_selected_cities(&cities, |city| city.monster_attack_risk > limit);
        expect(&n).equals(&1);
    }

    fn has_monster_attacks(city: &City) -> bool {
        city.monster_attack_risk > 0.0
    }

    #[test]
    fn count_selected_cities_has_monster_attacks_fn() {
        let cities = vec![
            City {
                name: "A".to_owned(),
                monster_attack_risk: 1.0,
            },
            City {
                name: "B".to_owned(),
                monster_attack_risk: 1.1,
            },
            City {
                name: "C".to_owned(),
                monster_attack_risk: 0.0,
            },
        ];

        let n = count_selected_cities(&cities, has_monster_attacks);
        expect(&n).equals(&2);
    }

    #[test]
    fn do_mutable_test() {
        let m_fn = |str: &mut String| {
            str.push_str(", World!");
            expect(str).equals(&"Hello, World!".to_owned())
        };
        do_mutable(m_fn);
    }
}
