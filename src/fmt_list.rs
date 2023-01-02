use crate::fmt_display;
use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a list of [`Debug`] objects as a list.
pub struct FmtDebugList<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDebugList<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtDebugList<F>
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

/// [`Debug`] or [`Display`] a list of [`Display`] objects as a list.
impl<F, I> Display for FmtDebugList<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// [`Debug`] or [`Display`] a list of [`Display`] objects as a list.
pub struct FmtDisplayList<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDisplayList<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtDisplayList<F>
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

impl<F, I> Display for FmtDisplayList<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Debug`] objects as a list.
pub const fn fmt_debug_list<F, I>(values_fn: F) -> FmtDebugList<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
{
    FmtDebugList::new(values_fn)
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Display`] objects as a list.
pub const fn fmt_display_list<F, I>(values_fn: F) -> FmtDisplayList<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
{
    FmtDisplayList::new(values_fn)
}

#[cfg(test)]
mod tests {
    use super::{FmtDebugList, FmtDisplayList};
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
            let fmt_list = super::fmt_debug_list(|| values);
            let unsized_fmt_list: &FmtDebugList<dyn Fn() -> &'static [Foo]> = &fmt_list;

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
            let fmt_list = super::fmt_display_list(|| values);
            let unsized_fmt_list: &FmtDisplayList<dyn Fn() -> &'static [Foo]> = &fmt_list;

            assert_eq!(std::format!("{}", fmt_list), expected);
            assert_eq!(std::format!("{}", unsized_fmt_list), expected);
        }
    }
}
