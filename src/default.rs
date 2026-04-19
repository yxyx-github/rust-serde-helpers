pub trait IsDefault: Default + PartialEq {
    fn is_default(&self) -> bool {
        *self == Default::default()
    }
}

impl<T: Default + PartialEq> IsDefault for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_default() {
        assert!(String::from("").is_default());
        assert!(!String::from("ABC").is_default());

        assert!(&String::from("").is_default());
        assert!(!&String::from("ABC").is_default());
    }
}