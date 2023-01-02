/// Like [`format_args`](`::core::format_args`), but takes ownership of its arguments.
///
/// Example:
///
/// ```rust
/// let fmt = {
///     let value_1 = vec![2];
///     let value_2 = Box::new('A');
///     let value_3 = Box::new(5);
///
///     fmt_tools::format_args_owned!("{:?}, {named}, {value_3}", value_1, named = value_2)
/// };
///
/// assert_eq!(format!("{fmt:?}"), "[2], A, 5");
/// assert_eq!(format!("{fmt}"), "[2], A, 5");
/// ```
#[macro_export]
macro_rules! format_args_owned {
    ($($args:tt)*) => {
        $crate::fmt_fn(
            move |__fmt_tools_formatter| ::core::fmt::Display::fmt(
                &::core::format_args!($($args)*),
                __fmt_tools_formatter,
            )
        )
    };
}
