use core::fmt::{self, Debug, Display, Formatter};

struct Inner<F, S>
where
    F: ?Sized,
{
    separator: S,
    values_fn: F,
}

impl<F, S> Inner<F, S>
where
    F: ?Sized,
{
    const fn new(values_fn: F, separator: S) -> Self
    where
        F: Sized,
    {
        Self { values_fn, separator }
    }

    fn fmt_with<I>(
        &self,
        f: &mut Formatter,
        value_writer: impl FnOnce(&I::Item, &mut Formatter) -> fmt::Result + Copy,
        separator_writer: impl FnOnce(&S, &mut Formatter) -> fmt::Result + Copy,
    ) -> fmt::Result
    where
        F: Fn() -> I,
        I: IntoIterator,
    {
        let mut iter = (self.values_fn)().into_iter();

        if let Some(first) = iter.next() {
            value_writer(&first, f)?;

            for item in iter {
                separator_writer(&self.separator, f)?;
                value_writer(&item, f)?;
            }
        }

        Ok(())
    }
}

/// [`Debug`] or [`Display`] a list of [`Debug`] objects, separating with another [`Debug`] object.
pub struct FmtSeparatedDebugList<F, S>
where
    F: ?Sized,
{
    inner: Inner<F, S>,
}

impl<F, S> FmtSeparatedDebugList<F, S> {
    const fn new(values_fn: F, separator: S) -> Self
    where
        F: Sized,
    {
        Self {
            inner: Inner::new(values_fn, separator),
        }
    }
}

impl<F, S, I> Debug for FmtSeparatedDebugList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.fmt_with(f, I::Item::fmt, S::fmt)
    }
}

impl<F, S, I> Display for FmtSeparatedDebugList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// [`Debug`] or [`Display`] a list of [`Display`] objects, separating with another [`Display`] object.
pub struct FmtSeparatedDisplayList<F, S>
where
    F: ?Sized,
{
    inner: Inner<F, S>,
}

impl<F, S> FmtSeparatedDisplayList<F, S> {
    const fn new(values_fn: F, separator: S) -> Self
    where
        F: Sized,
    {
        Self {
            inner: Inner::new(values_fn, separator),
        }
    }
}

impl<F, S, I> Debug for FmtSeparatedDisplayList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.fmt_with(f, I::Item::fmt, S::fmt)
    }
}

impl<F, S, I> Display for FmtSeparatedDisplayList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Debug`] objects, separating with `separator`.
pub const fn fmt_separated_debug_list<F, S, I>(values_fn: F, separator: S) -> FmtSeparatedDebugList<F, S>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
    S: Debug,
{
    FmtSeparatedDebugList::new(values_fn, separator)
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Display`] objects, separating with `separator`.
pub const fn fmt_separated_display_list<F, S, I>(values_fn: F, separator: S) -> FmtSeparatedDisplayList<F, S>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
    S: Display,
{
    FmtSeparatedDisplayList::new(values_fn, separator)
}

#[cfg(test)]
mod tests {
    use super::{FmtSeparatedDebugList, FmtSeparatedDisplayList};
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_separated_list() {
        #[derive(Debug)]
        struct Foo;

        #[derive(Debug)]
        struct Bar;

        let test_cases = [
            (&[] as &[Foo], ""),
            (&[Foo], "Foo"),
            (&[Foo, Foo], "FooBarFoo"),
            (&[Foo, Foo, Foo], "FooBarFooBarFoo"),
        ];

        for (values, expected) in test_cases {
            let fmt = super::fmt_separated_debug_list(|| values, Bar);
            let unsized_fmt: &FmtSeparatedDebugList<dyn Fn() -> &'static [Foo], Bar> = &fmt;

            assert_eq!(std::format!("{:?}", fmt), expected);
            assert_eq!(std::format!("{:?}", unsized_fmt), expected);
        }
    }

    #[test]
    fn test_display_separated_list() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("item")
            }
        }

        struct Bar;

        impl Display for Bar {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(", ")
            }
        }

        let test_cases = [
            (&[] as &[Foo], ""),
            (&[Foo], "item"),
            (&[Foo, Foo], "item, item"),
            (&[Foo, Foo, Foo], "item, item, item"),
        ];

        for (values, expected) in test_cases {
            let fmt = super::fmt_separated_display_list(|| values, Bar);
            let unsized_fmt: &FmtSeparatedDisplayList<dyn Fn() -> &'static [Foo], Bar> = &fmt;

            assert_eq!(std::format!("{}", fmt), expected);
            assert_eq!(std::format!("{}", unsized_fmt), expected);
        }
    }
}
