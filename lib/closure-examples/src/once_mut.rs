#[cfg(test)]
mod tests {
    use crate::once_mut::*;
    use expect::expect;
    #[test]
    fn fn_once() {
        let once = |str: &str| {
            expect(&str).equals(&"Hello");
        };

        given_fn_once(once);
    }

    #[test]
    fn fn_mut() {
        let mut x = 100u32;
        let mut_fn = |i: u32| {
            x += i;
            expect(&x).equals(&101);
        };

        given_fn_mut(mut_fn);
        // given_fn_mut(mut_fn); // FnMutはFnOnceのサブトレイトであるため、FnMutに属するクロージャーはFnOnceの要件を全て満たす
    }
}

pub fn given_fn_once<F>(f: F)
where
    F: FnOnce(&str),
{
    f("Hello");
    // f("World!"); // FnOnceは一度しか呼び出すことはできない
}

pub fn given_fn_mut<F>(mut f: F)
where
    F: FnMut(u32),
{
    f(1);
}
