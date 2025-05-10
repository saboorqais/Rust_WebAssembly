#[macro_export]
macro_rules! validate_or_return {
    ($validator:ty, $parts:expr) => {
        if let Err(e) = <$validator>::validate(&$parts) {
            return e;
        }
    };
}