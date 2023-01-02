use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a value based on a function.
pub struct FmtFn<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtFn<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F> Display for FmtFn<F>
where
    F: Fn(&mut Formatter) -> fmt::Result + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (self.values_fn)(f)
    }
}

impl<F> Debug for FmtFn<F>
where
    F: Fn(&mut Formatter) -> fmt::Result + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (self.values_fn)(f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a value based on the `f` function.
pub const fn fmt_fn<F>(f: F) -> FmtFn<F>
where
    F: Fn(&mut Formatter) -> fmt::Result,
{
    FmtFn::new(f)
}

#[cfg(test)]
mod tests {
    use super::FmtFn;
    use core::fmt::{self, Formatter};

    #[test]
    fn test_fmt_fn() {
        let fmt_fn = super::fmt_fn(|f| f.write_str("foo"));

        assert_eq!(std::format!("{fmt_fn:?}"), "foo");
        assert_eq!(std::format!("{fmt_fn}"), "foo");
    }

    #[test]
    fn test_coerce_unsized() {
        let fmt_fn: &FmtFn<dyn Fn(&mut Formatter) -> fmt::Result> = &super::fmt_fn(|f| f.write_str("foo"));

        assert_eq!(std::format!("{fmt_fn:?}"), "foo");
        assert_eq!(std::format!("{fmt_fn}"), "foo");
    }
}
