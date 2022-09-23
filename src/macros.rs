#[macro_export]
macro_rules! error_from {
    ($msg:literal $(,)?) => {
        $crate::Error::from_anyhow(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::Error::from_anyhow(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::from_anyhow(anyhow::anyhow!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return ::core::result::Result::Err($crate::error_from!($msg))
    };
    ($err:expr $(,)?) => {
        return ::core::result::Result::Err($crate::error_from!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return ::core::result::Result::Err($crate::error_from!($fmt, $($arg)*))
    };
}
