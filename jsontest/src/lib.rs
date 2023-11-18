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
        assert_eq!(r#"{"name":"Hello"}"#, serialized);
    }

    #[test]
    fn parse_ok() {
        let user: User = serde_json::from_str(r#"{"name":"Hello"}"#).unwrap();
        println!("{:?}", user);
        assert_eq!(user.name, "Hello");
        assert_eq!(user.email, None);
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
        assert_eq!(user.name, "Hello");
        assert_eq!(user.email.unwrap(), "hello@example.com");
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
        assert_eq!(user.name, "Hello");
        assert_eq!(user.extra["age"], 10);
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
        assert_eq!(user.name, "Hello");
        assert!(user.extra.contains_key("skills"));
    }
}
