# Struct ⇔ JSON

RustでJSONを扱う方法について学習したので記録しておく。

## 1. プロジェクト作成

```bash
$ cargo new jsontest --lib
$ cd jsontest
```

## 2. ライブラリ追加

```bash
$ cargo add serde --no-default-features --features derive
$ cargo add serde_json
```

##### [Cargo.toml](Cargo.toml)

```toml
[package]
name = "jsontest"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0.192", default-features = false, features = ["derive"] }
serde_json = "1.0.108"
```

## 3. プログラム作成

User構造体を定義して、JSONデータからUser構造体のオブジェクトへマッピングする方法と、その逆に、User構造体のオブジェクトからJSONデータを生成する方法をテストケースで示す。

User構造体に未定義な項目がJSONデータから渡ってきた場合や、必須項目としないOption型の扱い方、必須データがJSONデータに含まれていない以異常ケースも検証してみた。

##### [src/lib.rs](src/lib.rs)

```rust
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,

    // https://serde.rs/attr-skip-serializing.html
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    //https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        let user: User = User {
            name: "Hello".to_string(),
            email: None,
            extra: HashMap::new(),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!(serialized, r#"{"name":"Hello"}"#);
    }

    #[test]
    fn parse_ok() {
        let user: User = serde_json::from_str(r#"{"name":"Hello"}"#).unwrap();
        println!("{:?}", user);
        assert_eq!("Hello", user.name);
        assert_eq!(None, user.email);
        assert!(user.extra.is_empty());
    }

    #[test]
    fn parse_ok_option() {
        let user: User = serde_json::from_str(r#"
            {
                "name":"Hello",
                "email":"hello@example.com"
            }"#).unwrap();
            println!("{:?}", user);
        assert_eq!("Hello", user.name);
        assert_eq!("hello@example.com", user.email.unwrap());
        assert!(user.extra.is_empty());
    }

    #[test]
    fn parse_ok_extra() {
        let user: User = serde_json::from_str(r#"
            {
                "name":"Hello",
                "age":10
            }"#).unwrap();
        println!("{:?}", user);
        assert_eq!("Hello", user.name);
        assert_eq!(10, user.extra["age"]);
    }

    #[test]
    fn parse_ok_extra_complex() {
        let user: User = serde_json::from_str(r#"
            {
                "name":"Hello",
                "skills": [
                    {
                        "name":"Rust",
                        "year":1
                    }
                ]
            }"#).unwrap();
        println!("{:?}", user);
        assert_eq!("Hello", user.name);
        assert!(user.extra.contains_key("skills"));
    }
}
```

## 4. テスト実行

JavaのJunitなど（期待値、結果）の

https://users.rust-lang.org/t/assert-eq-expected-and-actual/20304

```bash
$ cargo test -- --nocapture
   Compiling jsontest v0.1.0 (/home/toshio/workspace/studyrust/jsontest)
    Finished test [unoptimized + debuginfo] target(s) in 0.31s
     Running unittests src/lib.rs (target/debug/deps/jsontest-ff95d53f666c330d)

running 6 tests
test tests::to_json ... ok
User { name: "Hello", email: None, extra: {} }
User { name: "Hello", email: Some("hello@example.com"), extra: {} }
User { name: "Hello", email: None, extra: {"age": Number(10)} }
User { name: "Hello", email: None, extra: {"skills": Array [Object {"name": String("Rust"), "year": Number(1)}]} }
test tests::parse_ok ... ok
test tests::parse_ok_option ... ok
test tests::parse_ok_extra ... ok
test tests::parse_ok_extra_complex ... ok
test tests::parse_err_no_required_field ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
