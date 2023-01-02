use crate::fmt_display;
use core::fmt::{self, Debug, Display, Formatter};

pub struct FmtSet<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtSet<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtSet<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)();

        f.debug_set().entries(entries).finish()
    }
}

impl<F, I> Display for FmtSet<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)().into_iter().map(fmt_display::fmt_display);

        f.debug_set().entries(entries).finish()
    }
}

pub const fn fmt_set<F, I>(values_fn: F) -> FmtSet<F>
where
    F: Fn() -> I,
    I: IntoIterator,
{
    FmtSet::new(values_fn)
}

pub const fn debug_fmt_set<F, I>(values_fn: F) -> FmtSet<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
{
    fmt_set(values_fn)
}

pub const fn display_fmt_set<F, I>(values_fn: F) -> FmtSet<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
{
    fmt_set(values_fn)
}

#[cfg(test)]
mod tests {
    use super::FmtSet;
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_fmt_set() {
        #[derive(Debug)]
        struct Foo;

        let test_cases = [
            (&[] as &[Foo], "{}"),
            (&[Foo], "{Foo}"),
            (&[Foo, Foo], "{Foo, Foo}"),
            (&[Foo, Foo, Foo], "{Foo, Foo, Foo}"),
        ];

        for (values, expected) in test_cases {
            let fmt_set = super::debug_fmt_set(|| values);
            let unsized_fmt_set: &FmtSet<dyn Fn() -> &'static [Foo]> = &fmt_set;

            assert_eq!(std::format!("{:?}", fmt_set), expected);
            assert_eq!(std::format!("{:?}", unsized_fmt_set), expected);
        }
    }

    #[test]
    fn test_display_fmt_set() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("item")
            }
        }

        let test_cases = [
            (&[] as &[Foo], "{}"),
            (&[Foo], "{item}"),
            (&[Foo, Foo], "{item, item}"),
            (&[Foo, Foo, Foo], "{item, item, item}"),
        ];

        for (values, expected) in test_cases {
            let fmt_set = super::display_fmt_set(|| values);
            let unsized_fmt_set: &FmtSet<dyn Fn() -> &'static [Foo]> = &fmt_set;

            assert_eq!(std::format!("{}", fmt_set), expected);
            assert_eq!(std::format!("{}", unsized_fmt_set), expected);
        }
    }
}
