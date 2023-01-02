use core::fmt::{self, Debug, Display, Formatter};

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
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Display for FmtDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

pub const fn fmt_display<T>(value: T) -> FmtDisplay<T> {
    FmtDisplay::new(value)
}

#[cfg(test)]
mod tests {
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_fmt_display() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("foo")
            }
        }

        let fmt_display = super::fmt_display(Foo);

        assert_eq!(std::format!("{:?}", fmt_display), "foo");
        assert_eq!(std::format!("{}", fmt_display), "foo");
    }
}
