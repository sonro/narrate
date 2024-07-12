/// Construct an ad-hoc error from a string or existing non-`narrate` error
/// value.
///
/// This evaluates to an [`Error`][crate::Error]. It can take either just a
/// string, or a format string with arguments. It also can take any custom type
/// which implements `Debug` and `Display`.
///
/// If called with a single argument whose type implements `std::error::Error`
/// (in addition to `Debug` and `Display`, which are always required), then that
/// Error impl's `source` is preserved as the `source` of the resulting
/// `narrate::Error`.
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use narrate::{error_from, Result};
///
/// fn lookup(key: &str) -> Result<V> {
///     if key.len() != 16 {
///         return Err(error_from!("key length must be 16 characters, got {:?}", key));
///     }
///
///     // ...
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! error_from {
    ($msg:literal $(,)?) => {
        $crate::Error::from_anyhow($crate::anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::Error::from_anyhow($crate::anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::from_anyhow($crate::anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// Return early with an error.
///
/// This macro is equivalent to `return Err(`[`error_from!($args...)`][error_from!]`)`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`narrate::Error`][crate::Error]`>`.
///
/// [error_from!]: crate::error_from
///
/// # Example
///
/// ```
/// # use narrate::{bail, Result};
/// #
/// # fn has_permission(user: usize, resource: usize) -> bool {
/// #     true
/// # }
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #     let resource = 0;
/// #
/// if !has_permission(user, resource) {
///     bail!("permission denied for accessing {}", resource);
/// }
/// #     Ok(())
/// # }
/// ```
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
