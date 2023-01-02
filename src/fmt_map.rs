use crate::{fmt_display, FmtDisplay};
use core::fmt::{self, Debug, Display, Formatter};

pub struct FmtMap<F>
where
    F: ?Sized,
{
    values_fn: F,
}

impl<F> FmtMap<F> {
    const fn new(values_fn: F) -> Self {
        Self { values_fn }
    }
}

impl<F, I, K, V> Debug for FmtMap<F>
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

impl<F, I, K, V> Display for FmtMap<F>
where
    F: Fn() -> I + ?Sized,
    I: IntoIterator<Item = (K, V)>,
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fn display_tuple_as_debug_tuple<K, V>((key, value): (K, V)) -> (FmtDisplay<K>, FmtDisplay<V>) {
            (fmt_display::fmt_display(key), fmt_display::fmt_display(value))
        }

        let entries = (self.values_fn)().into_iter().map(display_tuple_as_debug_tuple);

        f.debug_map().entries(entries).finish()
    }
}

pub const fn fmt_map<F, I, K, V>(values_fn: F) -> FmtMap<F>
where
    F: Fn() -> I,
    I: IntoIterator<Item = (K, V)>,
{
    FmtMap::new(values_fn)
}

pub const fn debug_fmt_map<F, I, K, V>(values_fn: F) -> FmtMap<F>
where
    F: Fn() -> I,
    I: IntoIterator<Item = (K, V)>,
    K: Debug,
    V: Debug,
{
    fmt_map(values_fn)
}

pub const fn display_fmt_map<F, I, K, V>(values_fn: F) -> FmtMap<F>
where
    F: Fn() -> I,
    I: IntoIterator<Item = (K, V)>,
    K: Display,
    V: Display,
{
    fmt_map(values_fn)
}

#[cfg(test)]
mod tests {
    use super::FmtMap;
    use core::fmt::{self, Display, Formatter};

    #[test]
    fn test_debug_fmt_map() {
        #[derive(Debug)]
        struct Foo;

        #[derive(Debug)]
        struct Bar;

        let test_cases = [
            (&[] as &[(Foo, Bar)], "{}"),
            (&[(Foo, Bar)], "{Foo: Bar}"),
            (&[(Foo, Bar), (Foo, Bar)], "{Foo: Bar, Foo: Bar}"),
            (&[(Foo, Bar), (Foo, Bar), (Foo, Bar)], "{Foo: Bar, Foo: Bar, Foo: Bar}"),
        ];

        for (values, expected) in test_cases {
            let fmt_map = super::debug_fmt_map(|| values.iter().map(|(key, value)| (key, value)));
            let unsized_fmt_map: &FmtMap<dyn Fn() -> _> = &fmt_map;

            assert_eq!(std::format!("{:?}", fmt_map), expected);
            assert_eq!(std::format!("{:?}", unsized_fmt_map), expected);
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

        let test_cases = [
            (&[] as &[(Foo, Bar)], "{}"),
            (&[(Foo, Bar)], "{foo: bar}"),
            (&[(Foo, Bar), (Foo, Bar)], "{foo: bar, foo: bar}"),
            (&[(Foo, Bar), (Foo, Bar), (Foo, Bar)], "{foo: bar, foo: bar, foo: bar}"),
        ];

        for (values, expected) in test_cases {
            let fmt_map = super::display_fmt_map(|| values.iter().map(|(key, value)| (key, value)));
            let unsized_fmt_map: &FmtMap<dyn Fn() -> _> = &fmt_map;

            assert_eq!(std::format!("{}", fmt_map), expected);
            assert_eq!(std::format!("{}", unsized_fmt_map), expected);
        }
    }
}
