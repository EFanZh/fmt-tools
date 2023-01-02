use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a value based on its [`Debug`] implementation.
pub struct FmtDebug<T>
where
    T: ?Sized,
{
    value: T,
}

impl<T> FmtDebug<T> {
    const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Debug for FmtDebug<T>
where
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Display for FmtDebug<T>
where
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a value based on its [`Debug`] implementation.
///
/// Example:
///
/// ```rust
/// #[derive(Debug)]
/// struct Foo;
///
/// let fmt = fmt_tools::fmt_debug(Foo);
///
/// assert_eq!(format!("{fmt:?}"), "Foo");
/// assert_eq!(format!("{fmt}"), "Foo");
/// ```
pub const fn fmt_debug<T>(value: T) -> FmtDebug<T>
where
    T: Debug,
{
    FmtDebug::new(value)
}

#[cfg(test)]
mod tests {
    use super::FmtDebug;
    use core::fmt::Debug;

    #[test]
    fn test_fmt_debug() {
        #[derive(Debug)]
        struct Foo;

        let fmt = super::fmt_debug(Foo);
        let unsized_fmt: &FmtDebug<dyn Debug> = &fmt;

        assert_eq!(std::format!("{fmt:?}"), "Foo");
        assert_eq!(std::format!("{fmt}"), "Foo");
        assert_eq!(std::format!("{unsized_fmt:?}"), "Foo");
        assert_eq!(std::format!("{unsized_fmt}"), "Foo");
    }
}
