// erverwhere use the `context!` macro to get a reference to the
// `ServiceContext` struct.
#[macro_export]
macro_rules! context {
    () => {
        &$crate::service::context()
    };
}

/// if Option is None or empty, return default
#[macro_export]
macro_rules! empty_or {
    ($s:expr, $default:expr) => {
        if !$s.as_ref().is_some_and(|x| x.len() != 0) {
            $default
        } else {
            $s.as_ref().unwrap()
        }
    };
}

/// check Option is empty
#[macro_export]
macro_rules! is_empty {
    ($s:expr) => {
        !$s.as_ref().is_some_and(|x| x.len() != 0)
    };
}
