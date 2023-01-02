use crate::{fmt_display, FmtDisplay};
use core::fmt::{self, Debug, Display, Formatter};

/// [`Debug`] or [`Display`] a list of `(Debug, Debug)` objects as a map.
pub struct FmtDebugMap<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDebugMap<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I, K, V> Debug for FmtDebugMap<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator<Item = (K, V)>,
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entries = (self.values_fn)();

        f.debug_map().entries(entries).finish()
    }
}

impl<F, I, K, V> Display for FmtDebugMap<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator<Item = (K, V)>,
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// [`Debug`] or [`Display`] a list of `(Display, Display)` objects as a map.
pub struct FmtDisplayMap<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtDisplayMap<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I, K, V> Debug for FmtDisplayMap<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator<Item = (K, V)>,
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fn display_as_debug<K, V>((key, value): (K, V)) -> (FmtDisplay<K>, FmtDisplay<V>)
        where
            K: Display,
            V: Display,
        {
            (fmt_display(key), fmt_display(value))
        }

        let entries = (self.values_fn)().into_iter().map(display_as_debug);

        f.debug_map().entries(entries).finish()
    }
}

impl<F, I, K, V> Display for FmtDisplayMap<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator<Item = (K, V)>,
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Creates an object that [`Debug`] or [`Display`] a list of `(Debug, Debug)` objects as a map.
///
/// Example:
///
/// ```rust
/// let fmt = fmt_tools::fmt_debug_map(|| ('a'..'d').zip('x'..));
///
/// assert_eq!(format!("{fmt:?}"), "{'a': 'x', 'b': 'y', 'c': 'z'}");
/// assert_eq!(format!("{fmt}"), "{'a': 'x', 'b': 'y', 'c': 'z'}");
/// ```
pub const fn fmt_debug_map<F, I, K, V>(values_fn: F) -> FmtDebugMap<F>
where
    F: Fn() -> I,
    I: IntoIterator<Item = (K, V)>,
    K: Debug,
    V: Debug,
{
    FmtDebugMap::new(values_fn)
}

/// Creates an object that [`Debug`] or [`Display`] a list of `(Display, Display)` objects as a map.
///
/// Example:
///
/// ```rust
/// let fmt = fmt_tools::fmt_display_map(|| ('a'..'d').zip('x'..));
///
/// assert_eq!(format!("{fmt:?}"), "{a: x, b: y, c: z}");
/// assert_eq!(format!("{fmt}"), "{a: x, b: y, c: z}");
/// ```
pub const fn fmt_display_map<F, I, K, V>(values_fn: F) -> FmtDisplayMap<F>
where
    F: Fn() -> I,
    I: IntoIterator<Item = (K, V)>,
    K: Display,
    V: Display,
{
    FmtDisplayMap::new(values_fn)
}

#[cfg(test)]
mod tests {
    use super::{FmtDebugMap, FmtDisplayMap};
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_fmt_map() {
        #[derive(Debug)]
        struct Foo;

        #[derive(Debug)]
        struct Bar;

        #[allow(trivial_casts)]
        let test_cases = [
            (&[] as &[(Foo, Bar)], "{}"),
            (&[(Foo, Bar)], "{Foo: Bar}"),
            (&[(Foo, Bar), (Foo, Bar)], "{Foo: Bar, Foo: Bar}"),
            (&[(Foo, Bar), (Foo, Bar), (Foo, Bar)], "{Foo: Bar, Foo: Bar, Foo: Bar}"),
        ];

        for (values, expected) in test_cases {
            let fmt_map = super::fmt_debug_map(|| values.iter().map(|(key, value)| (key, value)));
            let unsized_fmt_map: &FmtDebugMap<dyn Fn() -> _> = &fmt_map;

            assert_eq!(std::format!("{fmt_map:?}"), expected);
            assert_eq!(std::format!("{fmt_map}"), expected);
            assert_eq!(std::format!("{unsized_fmt_map:?}"), expected);
            assert_eq!(std::format!("{unsized_fmt_map}"), expected);
        }
    }

    #[test]
    fn test_display_fmt_map() {
        struct Foo;

        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("foo")
            }
        }

        struct Bar;

        impl Display for Bar {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("bar")
            }
        }

        #[allow(trivial_casts)]
        let test_cases = [
            (&[] as &[(Foo, Bar)], "{}"),
            (&[(Foo, Bar)], "{foo: bar}"),
            (&[(Foo, Bar), (Foo, Bar)], "{foo: bar, foo: bar}"),
            (&[(Foo, Bar), (Foo, Bar), (Foo, Bar)], "{foo: bar, foo: bar, foo: bar}"),
        ];

        for (values, expected) in test_cases {
            let fmt_map = super::fmt_display_map(|| values.iter().map(|(key, value)| (key, value)));
            let unsized_fmt_map: &FmtDisplayMap<dyn Fn() -> _> = &fmt_map;

            assert_eq!(std::format!("{fmt_map:?}"), expected);
            assert_eq!(std::format!("{fmt_map}"), expected);
            assert_eq!(std::format!("{unsized_fmt_map:?}"), expected);
            assert_eq!(std::format!("{unsized_fmt_map}"), expected);
        }
    }
}
