#[cfg(test)]

mod tests {
    use pinset_sync_rust::db::{get, set};

    #[test]
    fn test_set() {
        set("abc", &"123".to_string());
    }

    #[test]
    fn test_set_get() {
        set("abc", &"123".to_string());
        let v = get("abc").unwrap();
        println!("{:?}", v);
    }
}
