pub trait IsDefault: Default + PartialEq {
    fn is_default(&self) -> bool {
        self == &Default::default()
    }
}

impl<T: Default + PartialEq> IsDefault for T {}