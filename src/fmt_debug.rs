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
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Display for FmtDebug<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] `value` based on its [`Debug`] implementation.
pub const fn fmt_debug<T>(value: T) -> FmtDebug<T>
where
    T: Debug,
{
    FmtDebug::new(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fmt_debug() {
        #[derive(Debug)]
        struct Foo;

        let fmt_debug = super::fmt_debug(Foo);

        assert_eq!(std::format!("{fmt_debug:?}"), "Foo");
        assert_eq!(std::format!("{fmt_debug}"), "Foo");
    }
}
