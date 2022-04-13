use std::fmt::Debug;

pub fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        print!("{:?}", u)
    }
}

#[cfg(test)]
mod tests {
    use crate::made_iterator::*;
    use expect_rs::expect;

    #[test]
    fn default() {
        let v = vec![4, 20, 12, 8, 6];
        let mut iter = v.iter();
        expect(&iter.next()).is_some_equal_to(&&4);
        expect(&iter.next()).is_some_equal_to(&&20);
        expect(&iter.next()).is_some_equal_to(&&12);
        expect(&iter.next()).is_some_equal_to(&&8);
        expect(&iter.next()).is_some_equal_to(&&6);
        expect(&iter.next()).equals(&None)
    }

    #[test]
    fn b_tree() {
        use std::collections::BTreeSet;
        // HashSetの場合、イテレータの生成する順番が非決定的になるのでBTreeSetを使用する
        let mut favorites = BTreeSet::new();
        favorites.insert("Lucy in the Sky with Diamonds".to_string());
        favorites.insert("Liebesträume No. 3".to_string());

        let mut iter = favorites.iter();

        // 後ろから順番に返してくれる
        expect(&iter.next()).is_some_equal_to(&&"Liebesträume No. 3".to_string());
        expect(&iter.next()).is_some_equal_to(&&"Lucy in the Sky with Diamonds".to_string());
        expect(&iter.next()).equals(&None);
    }

    #[test]
    fn dump_test() {
        let v = vec![1, 2, 3, 4, 5];
        dump(v.iter());
    }
}
