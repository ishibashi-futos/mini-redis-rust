use std::collections::HashMap;
#[derive(Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[test]
fn make_json() {
    let mut json = HashMap::<String, Json>::new();
    let _entry = json
        .entry("key".to_string())
        .or_insert(Json::String("hoge".to_string()));
}
