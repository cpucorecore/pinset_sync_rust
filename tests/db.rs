mod common;

#[cfg(test)]
mod tests {
    use crate::common::setup;
    use pinset_sync_rust::db::{get, set};

    #[test]
    fn test_set() {
        setup();
        set("abc", &"123".to_string());
    }

    #[test]
    fn test_set_get() {
        setup();

        set("abc", &"123".to_string());
        let v = get("abc").unwrap();
        println!("{:?}", v);
    }
}
