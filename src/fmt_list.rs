use crate::fmt_display;
use core::fmt::{self, Debug, Display, Formatter};

pub struct FmtList<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtList<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtList<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)();

        f.debug_list().entries(entries).finish()
    }
}

impl<F, I> Display for FmtList<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)().into_iter().map(fmt_display::fmt_display);

        f.debug_list().entries(entries).finish()
    }
}

pub const fn fmt_list<F, I>(values_fn: F) -> FmtList<F>
where
    F: Fn() -> I,
    I: IntoIterator,
{
    FmtList::new(values_fn)
}

pub const fn debug_fmt_list<F, I>(values_fn: F) -> FmtList<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
{
    fmt_list(values_fn)
}

pub const fn display_fmt_list<F, I>(values_fn: F) -> FmtList<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
{
    fmt_list(values_fn)
}

#[cfg(test)]
mod tests {
    use super::FmtList;
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_fmt_list() {
        #[derive(Debug)]
        struct Foo;

        let test_cases = [
            (&[] as &[Foo], "[]"),
            (&[Foo], "[Foo]"),
            (&[Foo, Foo], "[Foo, Foo]"),
            (&[Foo, Foo, Foo], "[Foo, Foo, Foo]"),
        ];

        for (values, expected) in test_cases {
            let fmt_list = super::debug_fmt_list(|| values);
            let unsized_fmt_list: &FmtList<dyn Fn() -> &'static [Foo]> = &fmt_list;

            assert_eq!(std::format!("{:?}", fmt_list), expected);
            assert_eq!(std::format!("{:?}", unsized_fmt_list), expected);
        }
    }

    #[test]
    fn test_display_fmt_list() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("item")
            }
        }

        let test_cases = [
            (&[] as &[Foo], "[]"),
            (&[Foo], "[item]"),
            (&[Foo, Foo], "[item, item]"),
            (&[Foo, Foo, Foo], "[item, item, item]"),
        ];

        for (values, expected) in test_cases {
            let fmt_list = super::display_fmt_list(|| values);
            let unsized_fmt_list: &FmtList<dyn Fn() -> &'static [Foo]> = &fmt_list;

            assert_eq!(std::format!("{}", fmt_list), expected);
            assert_eq!(std::format!("{}", unsized_fmt_list), expected);
        }
    }
}
