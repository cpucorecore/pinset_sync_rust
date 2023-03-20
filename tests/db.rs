mod common;

#[cfg(test)]
mod tests {
    use crate::common::setup;
    use pinset_sync_rust::db::{pinset_get, pinset_set};

    #[test]
    fn test_set() {
        setup();
        pinset_set("abc", &"123".to_string());
    }

    #[test]
    fn test_set_get() {
        setup();

        pinset_set("abc", &"123".to_string());
        let v = pinset_get("abc").unwrap();
        println!("{:?}", v);
    }
}
