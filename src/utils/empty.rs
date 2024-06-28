pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => s.is_empty(),
            _ => true,
        }
    }
}

impl IsEmpty for Option<&str> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => s.is_empty(),
            _ => true,
        }
    }
}
