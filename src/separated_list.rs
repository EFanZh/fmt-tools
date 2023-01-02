use core::fmt::{self, Debug, Display, Formatter};

pub struct SeparatedList<F, S>
where
    F: ?Sized,
{
    separator: S,
    values_fn: F,
}

impl<F, S> SeparatedList<F, S>
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

impl<F, S, I> Debug for SeparatedList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_with(f, I::Item::fmt, S::fmt)
    }
}

impl<F, S, I> Display for SeparatedList<F, S>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
    S: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_with(f, I::Item::fmt, S::fmt)
    }
}

pub const fn separated_list<F, S, I>(values_fn: F, separator: S) -> SeparatedList<F, S>
where
    F: Fn() -> I,
    I: IntoIterator,
{
    SeparatedList::new(values_fn, separator)
}

pub const fn debug_separated_list<F, S, I>(values_fn: F, separator: S) -> SeparatedList<F, S>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
    S: Debug,
{
    separated_list(values_fn, separator)
}

pub const fn display_separated_list<F, S, I>(values_fn: F, separator: S) -> SeparatedList<F, S>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
    S: Display,
{
    separated_list(values_fn, separator)
}

#[cfg(test)]
mod tests {
    use super::SeparatedList;
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
            let separated_list = super::debug_separated_list(|| values, Bar);
            let unsized_separated_list: &SeparatedList<dyn Fn() -> &'static [Foo], Bar> = &separated_list;

            assert_eq!(std::format!("{:?}", separated_list), expected);
            assert_eq!(std::format!("{:?}", unsized_separated_list), expected);
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
            let separated_list = super::display_separated_list(|| values, Bar);
            let unsized_separated_list: &SeparatedList<dyn Fn() -> &'static [Foo], Bar> = &separated_list;

            assert_eq!(std::format!("{}", separated_list), expected);
            assert_eq!(std::format!("{}", unsized_separated_list), expected);
        }
    }
}
