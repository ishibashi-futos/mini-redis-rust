#[cfg(test)]
mod tests {
    use expect_rs::expect;

    #[test]
    fn map() {
        let message = "ponies \n giraffes\niguanas   \nsquid".to_string();

        let v = message.lines().map(str::trim).collect::<Vec<&str>>();

        expect(&v).contains_all(&vec![
            "ponies",
            "giraffes",
            "iguanas",
            "squid",
        ]);
    }
}
