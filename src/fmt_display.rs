use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a value based on its [`Display`] implementation.
pub struct FmtDisplay<T>
where
    T: ?Sized,
{
    value: T,
}

impl<T> FmtDisplay<T> {
    const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Debug for FmtDisplay<T>
where
    T: Display + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Display for FmtDisplay<T>
where
    T: Display + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a value based on its [`Display`] implementation.
///
/// Example:
///
/// ```rust
/// use std::fmt::{self, Display, Formatter};
///
/// struct Foo;
///
/// impl Display for Foo {
///     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
///         f.write_str("foo")
///     }
/// }
///
/// let fmt = fmt_tools::fmt_display(Foo);
///
/// assert_eq!(format!("{fmt:?}"), "foo");
/// assert_eq!(format!("{fmt}"), "foo");
/// ```
pub const fn fmt_display<T>(value: T) -> FmtDisplay<T>
where
    T: Display,
{
    FmtDisplay::new(value)
}

#[cfg(test)]
mod tests {
    use super::FmtDisplay;
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_fmt_display() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("foo")
            }
        }

        let fmt = super::fmt_display(Foo);
        let unsized_fmt: &FmtDisplay<dyn Display> = &fmt;

        assert_eq!(std::format!("{fmt:?}"), "foo");
        assert_eq!(std::format!("{fmt}"), "foo");
        assert_eq!(std::format!("{unsized_fmt:?}"), "foo");
        assert_eq!(std::format!("{unsized_fmt}"), "foo");
    }
}
