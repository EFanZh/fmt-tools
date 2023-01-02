use crate::fmt_display;
use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a list of [`Debug`] objects as a set.
pub struct FmtDebugSet<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDebugSet<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtDebugSet<F>
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

impl<F, I> Display for FmtDebugSet<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// [`Debug`] or [`Display`] a list of [`Display`] objects as a set.
pub struct FmtDisplaySet<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDisplaySet<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I> Debug for FmtDisplaySet<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)().into_iter().map(fmt_display);

        f.debug_set().entries(entries).finish()
    }
}

impl<F, I> Display for FmtDisplaySet<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator,
    I::Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Debug`] objects as a set.
///
/// Example:
///
/// ```rust
/// let fmt = fmt_tools::fmt_debug_set(|| 'a'..'g');
///
/// assert_eq!(format!("{fmt:?}"), "{'a', 'b', 'c', 'd', 'e', 'f'}");
/// assert_eq!(format!("{fmt}"), "{'a', 'b', 'c', 'd', 'e', 'f'}");
/// ```
pub const fn fmt_debug_set<F, I>(values_fn: F) -> FmtDebugSet<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Debug,
{
    FmtDebugSet::new(values_fn)
}

/// Creates an object that [`Debug`] or [`Display`] a list of [`Display`] objects as a set.
///
/// Example:
///
/// ```rust
/// let fmt = fmt_tools::fmt_display_set(|| 'a'..'g');
///
/// assert_eq!(format!("{fmt:?}"), "{a, b, c, d, e, f}");
/// assert_eq!(format!("{fmt}"), "{a, b, c, d, e, f}");
/// ```
pub const fn fmt_display_set<F, I>(values_fn: F) -> FmtDisplaySet<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: Display,
{
    FmtDisplaySet::new(values_fn)
}

#[cfg(test)]
mod tests {
    use super::{FmtDebugSet, FmtDisplaySet};
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_fmt_set() {
        #[derive(Debug)]
        struct Foo;

        #[allow(trivial_casts)]
        let test_cases = [
            (&[] as &[Foo], "{}"),
            (&[Foo], "{Foo}"),
            (&[Foo, Foo], "{Foo, Foo}"),
            (&[Foo, Foo, Foo], "{Foo, Foo, Foo}"),
        ];

        for (values, expected) in test_cases {
            let fmt = super::fmt_debug_set(|| values);
            let unsized_fmt: &FmtDebugSet<dyn Fn() -> &'static [Foo]> = &fmt;

            assert_eq!(std::format!("{fmt:?}"), expected);
            assert_eq!(std::format!("{fmt}"), expected);
            assert_eq!(std::format!("{unsized_fmt:?}"), expected);
            assert_eq!(std::format!("{unsized_fmt}"), expected);
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

        #[allow(trivial_casts)]
        let test_cases = [
            (&[] as &[Foo], "{}"),
            (&[Foo], "{item}"),
            (&[Foo, Foo], "{item, item}"),
            (&[Foo, Foo, Foo], "{item, item, item}"),
        ];

        for (values, expected) in test_cases {
            let fmt = super::fmt_display_set(|| values);
            let unsized_fmt: &FmtDisplaySet<dyn Fn() -> &'static [Foo]> = &fmt;

            assert_eq!(std::format!("{fmt:?}"), expected);
            assert_eq!(std::format!("{fmt}"), expected);
            assert_eq!(std::format!("{unsized_fmt:?}"), expected);
            assert_eq!(std::format!("{unsized_fmt}"), expected);
        }
    }
}
