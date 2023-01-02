use core::fmt::{self, Debug, Display, Formatter};

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

pub const fn fmt_debug<T>(value: T) -> FmtDebug<T> {
    FmtDebug::new(value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fmt_debug() {
        #[derive(Debug)]
        struct Foo;

        let fmt_debug = super::fmt_debug(Foo);

        assert_eq!(std::format!("{:?}", fmt_debug), "Foo");
        assert_eq!(std::format!("{}", fmt_debug), "Foo");
    }
}
